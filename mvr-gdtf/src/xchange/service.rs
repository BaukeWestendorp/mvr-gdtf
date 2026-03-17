use std::{
    collections::{HashMap, HashSet},
    io,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use mdns_sd::{ServiceEvent, ServiceInfo};
use uuid::Uuid;

use crate::xchange::packet::{Commit, Packet, PacketPayload};

pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";
const REGISTRATION_INTERVAL: Duration = Duration::from_secs(10);
const STALE_THRESHOLD: Duration = Duration::from_secs(30);

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
    settings: Settings,
    inner: Arc<Inner>,
    shutdown: Arc<AtomicBool>,
    handles: Mutex<Vec<JoinHandle<()>>>,
}

impl Service {
    pub fn new(mut settings: Settings) -> Result<Self, crate::xchange::Error> {
        // Sanitize the station name.
        settings.station_name = settings
            .station_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
            .collect();

        let inner = Arc::new(Inner::new(settings.clone())?);

        let this = Self {
            settings,
            inner,
            shutdown: Arc::new(AtomicBool::new(false)),
            handles: Mutex::new(Vec::new()),
        };

        this.start_listener()?;
        this.start_mdns_registration()?;
        this.start_mdns_browser_and_join()?;
        this.start_purger();

        Ok(this)
    }

    fn start_purger(&self) {
        let inner = Arc::clone(&self.inner);
        let shutdown = Arc::clone(&self.shutdown);

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                let mut slept = Duration::ZERO;
                while slept < Duration::from_secs(5) && !shutdown.load(Ordering::Relaxed) {
                    let step = Duration::from_millis(200);
                    thread::sleep(step);
                    slept += step;
                }
                if shutdown.load(Ordering::Relaxed) {
                    break;
                }
                inner.purge_once();
            }
        });

        self.handles.lock().unwrap().push(handle);
    }

    fn start_listener(&self) -> Result<(), io::Error> {
        let listener = TcpListener::bind((self.settings.interface_ip, self.settings.port))?;
        listener.set_nonblocking(true)?;
        let inner = Arc::clone(&self.inner);
        let shutdown = Arc::clone(&self.shutdown);

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _addr)) => {
                        let inner = Arc::clone(&inner);
                        let shutdown = Arc::clone(&shutdown);
                        thread::spawn(move || {
                            if shutdown.load(Ordering::Relaxed) {
                                return;
                            }
                            if let Err(err) = inner.handle_connection(stream) {
                                log::error!("failed to handle connection: {}", err);
                            }
                        });
                    }
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(err) => {
                        log::error!("failed to accept connection: {}", err);
                        thread::sleep(Duration::from_millis(200));
                    }
                }
            }
        });

        self.handles.lock().unwrap().push(handle);

        Ok(())
    }

    fn start_mdns_registration(&self) -> Result<(), crate::xchange::Error> {
        let mdns = mdns_sd::ServiceDaemon::new()?;
        let inner = Arc::clone(&self.inner);
        let shutdown = Arc::clone(&self.shutdown);

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                let settings = &inner.settings;
                let service_result = ServiceInfo::new(
                    SERVICE_TYPE,
                    &settings.group_name,
                    &format!("{}.local.", settings.station_name),
                    &settings.interface_ip.to_string(),
                    settings.port,
                    HashMap::from([
                        ("StationName".to_string(), settings.station_name.to_string()),
                        ("StationUUID".to_string(), settings.station_uuid.to_string()),
                    ]),
                );

                let Ok(service) = service_result else {
                    log::error!("failed to create mDNS service: {}", service_result.unwrap_err());
                    thread::sleep(Duration::from_millis(200));
                    continue;
                };

                if let Err(err) = mdns.register(service) {
                    log::error!("failed to register mDNS service: {err}");
                }

                let mut slept = Duration::ZERO;
                while slept < REGISTRATION_INTERVAL && !shutdown.load(Ordering::Relaxed) {
                    let step = Duration::from_millis(200);
                    thread::sleep(step);
                    slept += step;
                }
            }

            // Best-effort shutdown of the daemon
            let _ = mdns.shutdown();
        });

        self.handles.lock().unwrap().push(handle);

        Ok(())
    }

    fn start_mdns_browser_and_join(&self) -> Result<(), crate::xchange::Error> {
        let mdns = mdns_sd::ServiceDaemon::new()?;
        let browser = mdns.browse(SERVICE_TYPE)?;
        let inner = Arc::clone(&self.inner);
        let settings = self.settings.clone();
        let shutdown = Arc::clone(&self.shutdown);

        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                let event = match browser.recv_timeout(Duration::from_millis(250)) {
                    Ok(event) => event,
                    Err(_) => continue,
                };

                let ServiceEvent::ServiceResolved(service) = event else { continue };

                let Some(uuid_str) = service.get_property_val_str("StationUUID") else {
                    continue;
                };
                let Ok(uuid) = Uuid::parse_str(uuid_str) else { continue };

                // Prevent the service from attempting to handshake with itself.
                if uuid == settings.station_uuid {
                    continue;
                }

                if inner.refresh_station(&uuid) {
                    continue;
                }

                for addr in service.get_addresses() {
                    if shutdown.load(Ordering::Relaxed) {
                        break;
                    }

                    let socket_addr = SocketAddr::from((addr.to_ip_addr(), service.get_port()));

                    if let Err(err) = inner.send_join(socket_addr) {
                        log::warn!("failed to send MVR_JOIN to {socket_addr}: {err}");
                    }
                }
            }

            let _ = mdns.shutdown();
        });

        self.handles.lock().unwrap().push(handle);

        Ok(())
    }

    pub fn stations_in_group(&self) -> Vec<StationInfo> {
        let joined_uuids = self.inner.joined_stations.lock().unwrap();
        let stations = self.inner.stations.lock().unwrap();
        joined_uuids.iter().filter_map(|uuid| stations.get(uuid).cloned()).collect()
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);

        let mut handles = self.handles.lock().unwrap();
        for handle in handles.drain(..) {
            let _ = handle.join();
        }
    }
}

struct Inner {
    settings: Settings,
    stations: Mutex<HashMap<Uuid, StationInfo>>,
    joined_stations: Mutex<HashSet<Uuid>>,
}

impl Inner {
    pub fn new(settings: Settings) -> Result<Self, io::Error> {
        Ok(Self {
            settings,
            stations: Mutex::new(HashMap::new()),
            joined_stations: Mutex::new(HashSet::new()),
        })
    }

    pub fn purge_once(&self) {
        let now = Instant::now();
        let mut stations = self.stations.lock().unwrap();
        let mut joined = self.joined_stations.lock().unwrap();

        stations.retain(|uuid, info| {
            let is_alive = now.duration_since(info.last_seen) < STALE_THRESHOLD;
            if !is_alive {
                joined.remove(uuid);
                log::info!("Station {} ({}) timed out", info.name, uuid);
            }
            is_alive
        });
    }

    fn refresh_station(&self, uuid: &Uuid) -> bool {
        if let Some(station) = self.stations.lock().unwrap().get_mut(uuid) {
            station.last_seen = Instant::now();
            true
        } else {
            false
        }
    }

    fn send_join(&self, socket_addr: SocketAddr) -> Result<(), crate::xchange::Error> {
        let stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(800))?;

        Packet::new(
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
        )?
        .write(&stream)?;

        let packet = Packet::read(&stream)?;

        match packet.payload {
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
                self.join_station(StationInfo::new(
                    station_uuid,
                    station_name,
                    stream.peer_addr()?,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ));
                Ok(())
            }
            PacketPayload::JoinRet { ok: false, message, .. } => {
                Err(crate::xchange::Error::InvalidPacket { message })
            }
            _ => Err(crate::xchange::Error::UnexpectedPacket),
        }
    }

    fn handle_connection(&self, stream: TcpStream) -> Result<(), crate::xchange::Error> {
        let packet = Packet::read(&stream)?;

        match packet.payload {
            PacketPayload::Join {
                provider,
                station_name,
                station_uuid,
                ver_major,
                ver_minor,
                commits,
            } => {
                self.join_station(StationInfo::new(
                    station_uuid,
                    station_name,
                    stream.peer_addr()?,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ));

                Packet::new(
                    PacketPayload::JoinRet {
                        ok: true,
                        message: String::new(),
                        provider: self.settings.provider_name.clone(),
                        station_name: self.settings.station_name.clone(),
                        station_uuid: self.settings.station_uuid,
                        ver_major: self.settings.ver_major,
                        ver_minor: self.settings.ver_minor,
                        commits: Vec::new(),
                    },
                    0,
                    1,
                )?
                .write(&stream)?;
            }
            PacketPayload::Leave { from_station_uuid } => {
                self.leave_station(&from_station_uuid);

                Packet::new(PacketPayload::LeaveRet { ok: true, message: String::new() }, 0, 1)?
                    .write(&stream)?;
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
                if let Some(station) = self.stations.lock().unwrap().get_mut(&station_uuid) {
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

                Packet::new(PacketPayload::CommitRet { ok: true, message: String::new() }, 0, 1)?
                    .write(&stream)?;
            }
            PacketPayload::Request { .. } => todo!("handle MVR_REQUEST"),
            PacketPayload::NewSessionHost { .. } => todo!("handle MVR_NEW_SESSION_HOST"),
            _ => return Err(crate::xchange::Error::UnexpectedPacket),
        }

        Ok(())
    }

    fn join_station(&self, info: StationInfo) {
        self.joined_stations.lock().unwrap().insert(info.uuid);
        self.stations.lock().unwrap().insert(info.uuid, info);
    }

    fn leave_station(&self, uuid: &Uuid) {
        self.joined_stations.lock().unwrap().remove(uuid);
        self.stations.lock().unwrap().remove(uuid);
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

    pub fn address(&self) -> SocketAddr {
        self.address
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
