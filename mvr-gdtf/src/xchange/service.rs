use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use futures::{SinkExt as _, StreamExt as _};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
    task::JoinSet,
    time,
};
use tokio_util::{codec::Framed, sync::CancellationToken};
use uuid::Uuid;

use crate::xchange::packet::{Commit, Packet, PacketCodec, PacketPayload};

pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";
const REGISTRATION_INTERVAL: Duration = Duration::from_secs(10);
const STALE_THRESHOLD: Duration = Duration::from_secs(30);
const PURGE_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub provider_name: String,
    pub group_name: String,
    pub station_name: String,
    pub station_uuid: Uuid,

    pub ver_major: u32,
    pub ver_minor: u32,

    pub interface_ip: IpAddr,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider_name: "mvr-gdtf".to_string(),
            group_name: "Default".to_string(),
            station_name: env!("CARGO_PKG_NAME").parse().expect("should parse provider name"),
            station_uuid: Uuid::new_v4(),

            ver_major: env!("CARGO_PKG_VERSION_MAJOR").parse().expect("should parse major version"),
            ver_minor: env!("CARGO_PKG_VERSION_MINOR").parse().expect("should parse minor version"),

            interface_ip: local_ip_address::local_ip().expect("should get local ip address"),
            port: 48484,
        }
    }
}

pub struct Service {
    inner: Arc<Inner>,
    cancel: Mutex<CancellationToken>,
    tasks: Mutex<JoinSet<()>>,
}

impl Service {
    pub async fn new(mut settings: Settings) -> Result<Self, crate::xchange::Error> {
        // Sanitize the station name.
        settings.station_name = settings
            .station_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
            .collect();

        let inner = Arc::new(Inner::new(settings));
        let service = Self {
            inner,
            cancel: Mutex::new(CancellationToken::new()),
            tasks: Mutex::new(JoinSet::new()),
        };
        service.start().await?;
        Ok(service)
    }

    pub async fn start(&self) -> Result<(), crate::xchange::Error> {
        let mut cancel_guard = self.cancel.lock().unwrap();
        let mut tasks_guard = self.tasks.lock().unwrap();

        if !tasks_guard.is_empty() {
            log::warn!("Service::start() called but service is already running.");
            return Ok(());
        }

        let addr = (self.inner.settings.interface_ip, self.inner.settings.port);
        let listener = TcpListener::bind(addr).await?;
        let mdns = ServiceDaemon::new()?;

        let cancel = CancellationToken::new();
        let mut tasks = JoinSet::new();

        tasks.spawn(Self::run_listener(Arc::clone(&self.inner), listener, cancel.clone()));
        tasks.spawn(Self::run_mdns_registration(
            Arc::clone(&self.inner),
            mdns.clone(),
            cancel.clone(),
        ));
        tasks.spawn(Self::run_mdns_browser(Arc::clone(&self.inner), mdns, cancel.clone()));
        tasks.spawn(Self::run_purger(Arc::clone(&self.inner), cancel.clone()));

        *cancel_guard = cancel;
        *tasks_guard = tasks;
        Ok(())
    }

    /// Gracefully shuts down all background tasks and sends MVR_LEAVE to every
    /// known station.
    pub async fn shutdown(&self) {
        let cancel_guard = self.cancel.lock().unwrap();
        let mut tasks_guard = self.tasks.lock().unwrap();

        let _ = self.leave_all().await;
        cancel_guard.cancel();
        while let Some(result) = tasks_guard.join_next().await {
            if let Err(err) = result {
                if !err.is_cancelled() {
                    log::warn!("Task exited with error: {err}");
                }
            }
        }
        tasks_guard.abort_all();
    }

    pub async fn join(&self, uuid: Uuid) -> Result<(), crate::xchange::Error> {
        let addr = self.inner.station_addr(uuid).await?;
        self.inner.send_join(addr).await
    }

    pub async fn join_all(&self) -> Result<(), crate::xchange::Error> {
        for info in self.stations().await {
            if let Err(err) = self.join(info.uuid).await {
                log::warn!("Failed to join station {}: {}", info.uuid, err);
            }
        }
        Ok(())
    }

    pub async fn leave(&self, uuid: Uuid) -> Result<(), crate::xchange::Error> {
        let addr = self.inner.station_addr(uuid).await?;
        self.inner.send_leave(uuid, addr).await
    }

    pub async fn leave_all(&self) -> Result<(), crate::xchange::Error> {
        for info in self.stations().await {
            if let Err(err) = self.leave(info.uuid).await {
                log::warn!("Failed to leave station {}: {}", info.uuid, err);
            }
        }
        Ok(())
    }

    pub async fn stations(&self) -> Vec<StationInfo> {
        let registered = self.inner.registered_stations.read().await;
        let stations = self.inner.stations.read().await;
        registered.iter().filter_map(|uuid| stations.get(uuid).cloned()).collect()
    }

    async fn run_listener(inner: Arc<Inner>, listener: TcpListener, cancel: CancellationToken) {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                result = listener.accept() => {
                    match result {
                        Ok((stream, _addr)) => {
                            let inner = Arc::clone(&inner);
                            tokio::spawn(async move {
                                if let Err(err) = inner.handle_connection(stream).await {
                                    log::error!("failed to handle connection: {err}");
                                }
                            });
                        }
                        Err(err) => log::error!("failed to accept connection: {err}"),
                    }
                }
            }
        }
    }

    async fn run_mdns_registration(
        inner: Arc<Inner>,
        mdns: ServiceDaemon,
        cancel: CancellationToken,
    ) {
        let mut interval = time::interval(REGISTRATION_INTERVAL);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                _ = interval.tick() => {
                    let settings = &inner.settings;
                    let service_result = ServiceInfo::new(
                        SERVICE_TYPE,
                        &settings.group_name,
                        &format!("{}.local.", settings.station_name),
                        &settings.interface_ip.to_string(),
                        settings.port,
                        HashMap::from([
                            ("StationName".to_string(), settings.station_name.clone()),
                            ("StationUUID".to_string(), settings.station_uuid.to_string()),
                        ]),
                    );

                    match service_result {
                        Ok(service) => {
                            if let Err(err) = mdns.register(service) {
                                log::error!("failed to register mDNS service: {err}");
                            }
                        }
                        Err(err) => log::error!("failed to create mDNS service: {err}"),
                    }
                }
            }
        }

        let _ = mdns.shutdown();
    }

    async fn run_mdns_browser(inner: Arc<Inner>, mdns: ServiceDaemon, cancel: CancellationToken) {
        let browser = match mdns.browse(SERVICE_TYPE) {
            Ok(b) => b,
            Err(err) => {
                log::error!("failed to start mDNS browse: {err}");
                return;
            }
        };

        let own_uuid = inner.settings.station_uuid;

        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                event = browser.recv_async() => {
                    let Ok(event) = event else { break };
                    let ServiceEvent::ServiceResolved(service) = event else { continue };

                    let Some(uuid_str) = service.get_property_val_str("StationUUID") else {
                        continue;
                    };
                    let Ok(uuid) = Uuid::parse_str(uuid_str) else { continue };

                    // Never handshake with ourselves.
                    if uuid == own_uuid { continue; }

                    // If we already know this station just refresh its timestamp.
                    if inner.refresh_station(&uuid).await { continue; }

                    for addr in service.get_addresses() {
                        let socket_addr = SocketAddr::from((addr.to_ip_addr(), service.get_port()));
                        if let Err(err) = inner.send_join(socket_addr).await {
                            log::warn!("failed to send MVR_JOIN to {socket_addr}: {err}");
                        }
                    }
                }
            }
        }

        drop(browser);
        let _ = mdns.shutdown();
    }

    async fn run_purger(inner: Arc<Inner>, cancel: CancellationToken) {
        let mut interval = time::interval(PURGE_INTERVAL);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                _ = interval.tick() => inner.purge_stale().await,
            }
        }
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        self.cancel.lock().unwrap().cancel();
        self.tasks.lock().unwrap().abort_all();
    }
}

struct Inner {
    settings: Settings,
    stations: RwLock<HashMap<Uuid, StationInfo>>,
    registered_stations: RwLock<HashSet<Uuid>>,
}

impl Inner {
    fn new(settings: Settings) -> Self {
        Self {
            settings,
            stations: RwLock::new(HashMap::new()),
            registered_stations: RwLock::new(HashSet::new()),
        }
    }

    async fn purge_stale(&self) {
        let now = Instant::now();
        let mut stations = self.stations.write().await;
        let mut registered = self.registered_stations.write().await;

        stations.retain(|uuid, info| {
            let alive = now.duration_since(info.last_seen) < STALE_THRESHOLD;
            if !alive {
                registered.remove(uuid);
                log::info!("Station {} ({}) timed out", info.name, uuid);
            }
            alive
        });
    }

    async fn refresh_station(&self, uuid: &Uuid) -> bool {
        if let Some(station) = self.stations.write().await.get_mut(uuid) {
            station.last_seen = Instant::now();
            true
        } else {
            false
        }
    }

    async fn station_addr(&self, uuid: Uuid) -> Result<SocketAddr, crate::xchange::Error> {
        self.stations
            .read()
            .await
            .get(&uuid)
            .map(|info| info.address)
            .ok_or(crate::xchange::Error::StationNotFound { uuid })
    }

    async fn register_station(&self, info: StationInfo) {
        self.registered_stations.write().await.insert(info.uuid);
        self.stations.write().await.insert(info.uuid, info);
    }

    async fn unregister_station(&self, uuid: &Uuid) {
        self.registered_stations.write().await.remove(uuid);
        self.stations.write().await.remove(uuid);
    }

    async fn send_packet_and_recv(
        &self,
        socket_addr: SocketAddr,
        packet: Packet,
    ) -> Result<Packet, crate::xchange::Error> {
        let stream = time::timeout(Duration::from_secs(1), TcpStream::connect(socket_addr))
            .await
            .map_err(|_| crate::xchange::Error::Timeout)??;

        let mut framed = Framed::new(stream, PacketCodec);
        framed.send(packet).await?;
        let ret_packet = framed.next().await.ok_or(crate::xchange::Error::ConnectionClosed)??;
        Ok(ret_packet)
    }

    async fn send_join(&self, socket_addr: SocketAddr) -> Result<(), crate::xchange::Error> {
        let packet = Packet::new(
            PacketPayload::Join {
                provider: self.settings.provider_name.clone(),
                station_name: self.settings.station_name.clone(),
                station_uuid: self.settings.station_uuid,
                ver_major: self.settings.ver_major,
                ver_minor: self.settings.ver_minor,
                commits: Vec::new(),
            },
            0,
            1,
        )?;

        match self.send_packet_and_recv(socket_addr, packet).await?.payload {
            PacketPayload::JoinRet {
                ok: true,
                provider,
                station_name,
                station_uuid,
                ver_major,
                ver_minor,
                commits,
                ..
            } => {
                self.register_station(StationInfo::new(
                    station_uuid,
                    station_name,
                    socket_addr,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ))
                .await;
                Ok(())
            }
            PacketPayload::JoinRet { ok: false, message, .. } => {
                Err(crate::xchange::Error::InvalidPacket { message })
            }
            _ => Err(crate::xchange::Error::UnexpectedPacket),
        }
    }

    async fn send_leave(
        &self,
        uuid: Uuid,
        socket_addr: SocketAddr,
    ) -> Result<(), crate::xchange::Error> {
        let packet = Packet::new(
            PacketPayload::Leave { from_station_uuid: self.settings.station_uuid },
            0,
            1,
        )?;

        match self.send_packet_and_recv(socket_addr, packet).await?.payload {
            PacketPayload::LeaveRet { ok: true, .. } => {
                self.unregister_station(&uuid).await;
                Ok(())
            }
            PacketPayload::LeaveRet { ok: false, message, .. } => {
                Err(crate::xchange::Error::InvalidPacket { message })
            }
            _ => Err(crate::xchange::Error::UnexpectedPacket),
        }
    }

    async fn handle_connection(&self, stream: TcpStream) -> Result<(), crate::xchange::Error> {
        let peer_addr = stream.peer_addr()?;
        let mut framed = Framed::new(stream, PacketCodec);

        let packet = framed.next().await.ok_or(crate::xchange::Error::ConnectionClosed)??;

        let ret_packet = match packet.payload {
            PacketPayload::Join {
                provider,
                station_name,
                station_uuid,
                ver_major,
                ver_minor,
                commits,
            } => {
                self.register_station(StationInfo::new(
                    station_uuid,
                    station_name,
                    peer_addr,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ))
                .await;

                PacketPayload::JoinRet {
                    ok: true,
                    message: String::new(),
                    provider: self.settings.provider_name.clone(),
                    station_name: self.settings.station_name.clone(),
                    station_uuid: self.settings.station_uuid,
                    ver_major: self.settings.ver_major,
                    ver_minor: self.settings.ver_minor,
                    commits: Vec::new(),
                }
            }

            PacketPayload::Leave { from_station_uuid } => {
                self.unregister_station(&from_station_uuid).await;
                PacketPayload::LeaveRet { ok: true, message: String::new() }
            }

            PacketPayload::Commit {
                file_uuid,
                station_uuid,
                file_size,
                ver_major,
                ver_minor,
                for_stations_uuid,
                file_name,
                comment,
            } => {
                if let Some(station) = self.stations.write().await.get_mut(&station_uuid) {
                    station.last_seen = Instant::now();
                    station.commits.push(Commit {
                        file_uuid,
                        station_uuid,
                        file_size,
                        ver_major,
                        ver_minor,
                        for_stations_uuid,
                        file_name,
                        comment,
                    });
                }

                PacketPayload::CommitRet { ok: true, message: String::new() }
            }

            PacketPayload::Request { .. } => todo!("handle MVR_REQUEST"),
            PacketPayload::NewSessionHost { .. } => todo!("handle MVR_NEW_SESSION_HOST"),
            _ => return Err(crate::xchange::Error::UnexpectedPacket),
        };

        framed.send(Packet::new(ret_packet, 0, 1)?).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationInfo {
    uuid: Uuid,
    name: String,
    address: SocketAddr,
    provider: String,
    ver_major: u32,
    ver_minor: u32,
    commits: Vec<Commit>,
    last_seen: Instant,
}

impl StationInfo {
    fn new(
        uuid: Uuid,
        name: String,
        address: SocketAddr,
        provider: String,
        ver_major: u32,
        ver_minor: u32,
        commits: Vec<Commit>,
    ) -> Self {
        Self {
            uuid,
            name,
            address,
            provider,
            ver_major,
            ver_minor,
            commits,
            last_seen: Instant::now(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn ver_major(&self) -> u32 {
        self.ver_major
    }
    pub fn ver_minor(&self) -> u32 {
        self.ver_minor
    }
    pub fn commits(&self) -> &[Commit] {
        &self.commits
    }
}
