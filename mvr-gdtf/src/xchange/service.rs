use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};

use futures_util::{SinkExt as _, StreamExt as _};
use mdns_sd::{ResolvedService, ServiceEvent, ServiceInfo};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
    time,
};
use tokio_util::codec::Framed;
use uuid::Uuid;

use crate::{
    mvr::MvrFile,
    xchange::{
        Error, StationInfo,
        packet::{Commit, Packet, PacketCodec, PacketPayload},
    },
};

/// The mDNS service type used to register and discover MVR-xchange stations.
pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";

const REGISTRATION_INTERVAL: Duration = Duration::from_secs(10);
const STALE_THRESHOLD: Duration = Duration::from_secs(30);
const PURGE_INTERVAL: Duration = Duration::from_secs(5);

const SUPPORTED_MVR_FILE_VERSION_MAJOR: u32 = 1;
const SUPPORTED_MVR_FILE_VERSION_MINOR: u32 = 6;

type StationEventHandler = Arc<dyn Fn(StationInfo) + Send + Sync + 'static>;

/// Configuration for a MVR-xchange station.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// Name of the application or tool.
    pub provider_name: String,
    /// Group name used in the mDNS service registration.
    pub group_name: String,
    /// Human-readable name for this station.
    pub station_name: String,
    /// Stable unique identifier for this station. It's recommended to keep this
    /// the same over multiple runs of the application.
    pub station_uuid: Uuid,

    /// Local IP address to bind the TCP listener and advertise over mDNS.
    pub interface_ip: IpAddr,
    /// TCP port to listen on.
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider_name: "mvr-gdtf".to_string(),
            group_name: "Default".to_string(),
            station_name: env!("CARGO_PKG_NAME").parse().expect("Should parse provider name"),
            station_uuid: Uuid::new_v4(),

            interface_ip: local_ip_address::local_ip().expect("Should get local ip address"),
            port: 48484,
        }
    }
}

/// Commands sent from a service handle to the background task.
enum Command {
    LoadLocalMvrFile {
        mvr_file: MvrFile,
    },
    GetLocalMvrFiles {
        resp: oneshot::Sender<Vec<Arc<MvrFile>>>,
    },
    Join {
        target_uuid: Uuid,
        resp: oneshot::Sender<Result<(), Error>>,
    },
    JoinAll {
        resp: oneshot::Sender<Result<(), Error>>,
    },
    Leave {
        target_uuid: Uuid,
        resp: oneshot::Sender<Result<(), Error>>,
    },
    LeaveAll {
        resp: oneshot::Sender<Result<(), Error>>,
    },
    Commit {
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
        resp: oneshot::Sender<Result<(), Error>>,
    },
    CommitAll {
        file_uuid: Uuid,
        comment: Option<String>,
        resp: oneshot::Sender<Result<(), Error>>,
    },
    Request {
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
        resp: oneshot::Sender<Result<Option<RequestedFile>, Error>>,
    },
    RequestAll {
        resp: oneshot::Sender<Result<Vec<RequestedFile>, Error>>,
    },
    Stations {
        resp: oneshot::Sender<Vec<StationInfo>>,
    },
    SetOnJoin(StationEventHandler),
    SetOnLeave(StationEventHandler),
    Shutdown {
        resp: oneshot::Sender<()>,
    },
}

/// Events sent internally from inbound TCP connection tasks to the event loop.
enum InternalEvent {
    InboundPacket {
        payload: PacketPayload,
        addr: SocketAddr,
        resp: oneshot::Sender<Result<PacketPayload, Error>>,
    },
}

/// A cloneable handle to a MVR-xchange station service.
///
/// Cheap to clone, as it's just a handle to the running background service.
/// The task starts on construction and shuts down either when
/// [`Service::shutdown`] is called or when all handles are dropped.
#[derive(Clone)]
pub struct Service {
    sender: mpsc::Sender<Command>,
}

impl Service {
    /// Creates a new MVR-xchange station and immediately starts the service.
    ///
    /// # Errors
    /// Returns an error if the TCP listener or mDNS daemon cannot be initialised.
    pub fn new(mut settings: Settings) -> Result<Self, Error> {
        // Sanitize the station name.
        settings.station_name = settings
            .station_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
            .collect();

        let (sender, receiver) = mpsc::channel(32);

        tokio::spawn(async move {
            if let Err(err) = Inner::new(settings).run(receiver).await {
                log::warn!("{err}");
            }
        });

        Ok(Self { sender })
    }

    async fn send_cmd(&self, cmd: Command) -> Result<(), Error> {
        self.sender.send(cmd).await.map_err(|_| Error::Shutdown)
    }

    /// Loads the MVR file into the pool, and sends `MVR_COMMIT`
    /// for `mvr_file` to every currently known station.
    ///
    /// # Errors
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn load_local_mvr_file(&self, mvr_file: MvrFile) -> Result<(), Error> {
        self.send_cmd(Command::LoadLocalMvrFile { mvr_file }).await
    }

    /// Returns all locally stored MVR files.
    /// # Errors
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn local_mvr_files(&self) -> Result<Vec<Arc<MvrFile>>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::GetLocalMvrFiles { resp: tx }).await?;
        Ok(rx.await.map_err(|_| Error::Shutdown)?)
    }

    /// Sends `MVR_JOIN` to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn join(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::Join { target_uuid, resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_JOIN` to every currently known station.
    ///
    /// # Errors
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn join_all(&self) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::JoinAll { resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_LEAVE` to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn leave(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::Leave { target_uuid, resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_LEAVE` to every currently known station.
    ///
    /// # Errors
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn leave_all(&self) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::LeaveAll { resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_COMMIT` for the MVR file associated with `file_uuid`
    /// to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn commit(
        &self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::Commit {
            target_uuid,
            file_uuid,
            comment: comment.map(Into::into),
            resp: tx,
        })
        .await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_COMMIT` for the MVR file associated with `file_uuid`
    /// to every currently known station.
    ///
    /// # Errors
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn commit_all(
        &self,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::CommitAll { file_uuid, comment: comment.map(Into::into), resp: tx })
            .await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_REQUEST` to request the file with UUID `file_uuid`.
    ///
    /// Returns the [`RequestedFile`] containing bytes of the compressed MVR file,
    /// or [`None`] if no matching file was found on any of the targeted stations.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn request(
        &self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
    ) -> Result<Option<RequestedFile>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::Request { file_uuid, from_station_uuid, resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sends `MVR_REQUEST` to every currently known station to request
    /// all their MVR files.
    ///
    /// Contacts each station in turn and collects their responses.
    /// Stations that fail or return no file are logged and skipped,
    /// so the map may contain fewer entries than there are known stations.
    ///
    /// # Errors
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn request_all(&self) -> Result<Vec<RequestedFile>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::RequestAll { resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)?
    }

    /// Sets the handler called whenever a station joins the network.
    ///
    /// Replaces any previously set handler.
    ///
    /// # Errors
    /// Returns [`Error::Shutdown`] if the
    /// background task has stopped.
    pub async fn on_join<F>(&self, handler: F) -> Result<(), Error>
    where
        F: Fn(StationInfo) + Send + Sync + 'static,
    {
        self.send_cmd(Command::SetOnJoin(Arc::new(handler))).await
    }

    /// Sets the handler called whenever a station leaves the network.
    ///
    /// # Errors
    /// Replaces any previously set handler. Returns [`Error::Shutdown`] if the
    /// background task has stopped.
    pub async fn on_leave<F>(&self, handler: F) -> Result<(), Error>
    where
        F: Fn(StationInfo) + Send + Sync + 'static,
    {
        self.send_cmd(Command::SetOnLeave(Arc::new(handler))).await
    }

    /// Returns a snapshot of all currently known stations.
    ///
    /// # Errors
    /// Returns [`Error::Shutdown`] if the background task has stopped.
    pub async fn stations(&self) -> Result<Vec<StationInfo>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send_cmd(Command::Stations { resp: tx }).await?;
        rx.await.map_err(|_| Error::Shutdown)
    }

    /// Gracefully shuts down the background task, sending `MVR_LEAVE` to all
    /// known stations, and waits for it to complete.
    ///
    /// Dropping all handles also triggers a shutdown, but without the ability
    /// to await its completion.
    pub async fn shutdown(&self) {
        let (tx, rx) = oneshot::channel();
        if self.send_cmd(Command::Shutdown { resp: tx }).await.is_ok() {
            let _ = rx.await;
        }
    }
}

struct Inner {
    settings: Settings,

    stations: HashMap<Uuid, StationInfo>,
    /// An inactive station is a station that has joined before (meaning we know them),
    /// but then left again. We keep them around to prevent reconnecting to them when
    /// we resolve mDNS services.
    inactive_stations: HashMap<Uuid, StationInfo>,
    local_mvr_files: HashMap<Uuid, Arc<MvrFile>>,

    on_join: Option<StationEventHandler>,
    on_leave: Option<StationEventHandler>,
}

impl Inner {
    fn new(settings: Settings) -> Self {
        Self {
            settings,
            stations: HashMap::new(),
            inactive_stations: HashMap::new(),
            local_mvr_files: HashMap::new(),

            on_join: None,
            on_leave: None,
        }
    }

    async fn run(mut self, mut receiver: mpsc::Receiver<Command>) -> Result<(), Error> {
        let listener = TcpListener::bind((self.settings.interface_ip, self.settings.port)).await?;

        let mdns_daemon = mdns_sd::ServiceDaemon::new()?;
        let mdns_browser = mdns_daemon.browse(SERVICE_TYPE)?;
        let mut mdns_register_interval = tokio::time::interval(REGISTRATION_INTERVAL);
        let mut purge_interval = tokio::time::interval(PURGE_INTERVAL);

        let (internal_tx, mut internal_rx) = mpsc::channel::<InternalEvent>(32);

        // We have to store the shutdown response channel here, because we need to
        // respond AFTER we have tried to send `MVR_LEAVE` to all stations.
        let mut shutdown_resp = None;

        loop {
            tokio::select! {
                cmd = receiver.recv() => match cmd {
                    Some(Command::Shutdown { resp }) => {
                        shutdown_resp = Some(resp);
                        break;
                    }
                    Some(cmd) => self.handle_command(cmd).await,
                    None => break, // All service handles were dropped.
                },

                event = internal_rx.recv() => {
                    if let Some(InternalEvent::InboundPacket { payload, addr, resp }) = event {
                        let _ = resp.send(self.handle_inbound_packet(payload, addr));
                    }
                }

                conn = listener.accept() => match conn {
                    Ok((conn, addr)) => {
                        let task_tx = internal_tx.clone();
                        tokio::spawn(async move {
                            if let Err(err) = handle_inbound(conn, addr, task_tx).await {
                                log::warn!("Failed to handle inbound connection {addr}: {err}");
                            }
                        });
                    }
                    Err(err) => log::warn!("Failed to accept connection: {err}"),
                },

                _ = mdns_register_interval.tick() => self.register_mdns_service(&mdns_daemon).await,

                service = mdns_browser.recv_async() => match service {
                    Ok(ServiceEvent::ServiceResolved(service)) => {
                        self.handle_resolved_mdns_service(&service).await;
                    }
                    Err(err) => log::warn!("Error while browsing for mDNS services: {err}"),
                    _ => {}
                },

                _ = purge_interval.tick() => self.purge_stale_stations(),
            }
        }

        let addrs: Vec<SocketAddr> = self.stations.values().map(|s| s.address).collect();
        for addr in addrs {
            if let Err(err) = send_leave(addr, self.settings.station_uuid).await {
                log::warn!("Failed to leave station while stopping service: {err}");
            }
        }

        if let Some(shutdown_resp) = shutdown_resp {
            let _ = shutdown_resp.send(());
        }

        Ok(())
    }

    async fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command::LoadLocalMvrFile { mvr_file } => {
                self.local_mvr_files.insert(mvr_file.file_hash_uuid(), Arc::new(mvr_file));
            }
            Command::GetLocalMvrFiles { resp } => {
                let local_mvr_files = self.local_mvr_files.values().cloned().collect();
                let _ = resp.send(local_mvr_files);
            }
            Command::Join { target_uuid, resp } => {
                let _ = resp.send(self.do_join(target_uuid).await);
            }
            Command::JoinAll { resp } => {
                let _ = resp.send(self.do_join_all().await);
            }
            Command::Leave { target_uuid, resp } => {
                let _ = resp.send(self.do_leave(target_uuid).await);
            }
            Command::LeaveAll { resp } => {
                let _ = resp.send(self.do_leave_all().await);
            }
            Command::Commit { target_uuid, file_uuid, comment, resp } => {
                let _ = resp.send(self.do_commit(target_uuid, file_uuid, comment).await);
            }
            Command::CommitAll { file_uuid, comment, resp } => {
                let _ = resp.send(self.do_commit_all(file_uuid, comment).await);
            }
            Command::Request { file_uuid, from_station_uuid, resp } => {
                let _ = resp.send(self.do_request(file_uuid, from_station_uuid).await);
            }
            Command::RequestAll { resp } => {
                let _ = resp.send(self.do_request_all().await);
            }
            Command::Stations { resp } => {
                // Only return active stations.
                let _ = resp.send(self.stations.values().cloned().collect());
            }
            Command::SetOnJoin(handler) => self.on_join = Some(handler),
            Command::SetOnLeave(handler) => self.on_leave = Some(handler),
            Command::Shutdown { .. } => unreachable!("Shutdown is handled in run()"),
        }
    }

    fn handle_inbound_packet(
        &mut self,
        payload: PacketPayload,
        addr: SocketAddr,
    ) -> Result<PacketPayload, Error> {
        let ret = match payload {
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
                    addr,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ));
                PacketPayload::JoinRet {
                    ok: true,
                    message: String::new(),
                    provider: self.settings.provider_name.clone(),
                    station_name: self.settings.station_name.clone(),
                    station_uuid: self.settings.station_uuid,
                    ver_major: SUPPORTED_MVR_FILE_VERSION_MAJOR,
                    ver_minor: SUPPORTED_MVR_FILE_VERSION_MINOR,
                    commits: self.local_commits(),
                }
            }

            PacketPayload::Leave { from_station_uuid } => {
                // The peer explicitly indicated it's leaving *us*.
                // Mark it as inactive so we keep metadata around but won't treat it as "new" via mDNS.
                self.unregister_station(&from_station_uuid);
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
                if let Some(station) = self.stations.get_mut(&station_uuid) {
                    station.last_seen = Instant::now();

                    let targeted_at_us = for_stations_uuid.is_empty()
                        || for_stations_uuid.contains(&self.settings.station_uuid);

                    if targeted_at_us {
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
                }
                PacketPayload::CommitRet { ok: true, message: String::new() }
            }

            PacketPayload::Request { .. } => todo!("handle MVR_REQUEST"),
            PacketPayload::NewSessionHost { .. } => todo!("handle MVR_NEW_SESSION_HOST"),
            _ => return Err(Error::UnexpectedPacket),
        };

        Ok(ret)
    }

    async fn register_mdns_service(&self, mdns_daemon: &mdns_sd::ServiceDaemon) {
        let service = ServiceInfo::new(
            SERVICE_TYPE,
            &self.settings.group_name,
            &format!("{}.local.", self.settings.station_name),
            &self.settings.interface_ip.to_string(),
            self.settings.port,
            HashMap::from([
                ("StationName".to_string(), self.settings.station_name.clone()),
                ("StationUUID".to_string(), self.settings.station_uuid.to_string()),
            ]),
        );

        match service {
            Ok(service) => {
                if let Err(err) = mdns_daemon.register(service) {
                    log::error!("Failed to register mDNS service: {err}");
                }
            }
            Err(err) => log::error!("Failed to create mDNS service: {err}"),
        }
    }

    async fn handle_resolved_mdns_service(&mut self, service: &ResolvedService) {
        let own_uuid = self.settings.station_uuid;

        let Some(uuid_str) = service.get_property_val_str("StationUUID") else {
            log::warn!("Resolved mDNS service does not have a StationUUID");
            return;
        };
        let Ok(uuid) = Uuid::parse_str(uuid_str) else {
            log::warn!("Resolved mDNS service has an invalid StationUUID: '{uuid_str}'");
            return;
        };

        if uuid == own_uuid {
            return;
        }

        if let Some(station) = self.stations.get_mut(&uuid) {
            // Already known & active, just refresh the timestamp.
            station.last_seen = Instant::now();
            return;
        }
        if self.inactive_stations.contains_key(&uuid) {
            return;
        }

        for addr in service.get_addresses() {
            let socket_addr = SocketAddr::from((addr.to_ip_addr(), service.get_port()));
            if let Err(err) = self.do_join_addr(socket_addr).await {
                log::warn!("Failed to handshake with {socket_addr}: {err}");
            }
        }
    }

    fn purge_stale_stations(&mut self) {
        let now = Instant::now();
        let stale_uuids: Vec<Uuid> = self
            .stations
            .values()
            .filter(|info| now.duration_since(info.last_seen) >= STALE_THRESHOLD)
            .map(|info| info.uuid())
            .collect();

        for uuid in stale_uuids {
            self.unregister_station(&uuid);
        }
    }

    fn station_addr(&self, uuid: Uuid) -> Result<SocketAddr, Error> {
        self.stations.get(&uuid).map(|info| info.address).ok_or(Error::StationNotFound { uuid })
    }

    async fn do_join(&mut self, target_uuid: Uuid) -> Result<(), Error> {
        let addr = self.station_addr(target_uuid)?;
        self.do_join_addr(addr).await
    }

    async fn do_join_all(&mut self) -> Result<(), Error> {
        let stations: Vec<(Uuid, String)> =
            self.stations.values().map(|s| (s.uuid(), s.name().to_string())).collect();

        for (uuid, name) in stations {
            if let Err(err) = self.do_join(uuid).await {
                log::warn!("Failed to join station '{name}' ({uuid}): {err}");
            }
        }
        Ok(())
    }

    async fn do_join_addr(&mut self, addr: SocketAddr) -> Result<(), Error> {
        let packet = Packet::new(
            PacketPayload::Join {
                provider: self.settings.provider_name.clone(),
                station_name: self.settings.station_name.clone(),
                station_uuid: self.settings.station_uuid,
                ver_major: SUPPORTED_MVR_FILE_VERSION_MAJOR,
                ver_minor: SUPPORTED_MVR_FILE_VERSION_MINOR,
                commits: self.local_commits(),
            },
            0,
            1,
        )?;

        match send_packet_and_recv(addr, packet).await?.payload {
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
                    addr,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                ));
                Ok(())
            }
            PacketPayload::JoinRet { ok: false, message, .. } => {
                Err(Error::InvalidPacket { message })
            }
            _ => Err(Error::UnexpectedPacket),
        }
    }

    async fn do_leave(&mut self, target_uuid: Uuid) -> Result<(), Error> {
        let addr = self.station_addr(target_uuid)?;
        send_leave(addr, self.settings.station_uuid).await?;
        self.unregister_station(&target_uuid);
        Ok(())
    }

    async fn do_leave_all(&mut self) -> Result<(), Error> {
        let stations: Vec<(Uuid, String)> =
            self.stations.values().map(|s| (s.uuid(), s.name().to_string())).collect();

        for (uuid, name) in stations {
            if let Err(err) = self.do_leave(uuid).await {
                log::warn!("Failed to leave station '{name}' ({uuid}): {err}");
            }
        }
        Ok(())
    }

    async fn do_commit(
        &mut self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
    ) -> Result<(), Error> {
        let addr = self.station_addr(target_uuid)?;
        let Some(mut commit) = self.local_commit(&file_uuid) else {
            return Err(Error::LocalMvrFileNotFound { uuid: file_uuid })?;
        };
        commit.comment = comment;
        send_commit(addr, commit.clone()).await
    }

    async fn do_commit_all(
        &mut self,
        file_uuid: Uuid,
        comment: Option<String>,
    ) -> Result<(), Error> {
        let stations: Vec<(Uuid, SocketAddr)> =
            self.stations.values().map(|s| (s.uuid(), s.address)).collect();
        let Some(mut commit) = self.local_commit(&file_uuid) else {
            return Err(Error::LocalMvrFileNotFound { uuid: file_uuid })?;
        };
        commit.comment = comment;
        for (uuid, addr) in stations {
            if let Err(err) = send_commit(addr, commit.clone()).await {
                log::warn!("Failed to send commit to station {uuid}: {err}");
            }
        }
        Ok(())
    }

    async fn do_request(
        &mut self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
    ) -> Result<Option<RequestedFile>, Error> {
        for &station_uuid in &from_station_uuid {
            let addr = self.station_addr(station_uuid)?;
            if let Some(bytes) = send_request(addr, file_uuid, from_station_uuid.clone()).await? {
                let file_name = self
                    .stations
                    .get(&station_uuid)
                    .and_then(|s| s.commits().iter().find_map(|c| c.file_name.to_owned()))
                    .unwrap_or(format!("{}.mvr", file_uuid.unwrap_or_default()));

                return Ok(Some(RequestedFile { uuid: station_uuid, name: file_name, bytes }));
            }
        }
        Ok(None)
    }

    async fn do_request_all(&mut self) -> Result<Vec<RequestedFile>, Error> {
        let mut files = Vec::new();

        let from_station_uuid = self.stations.keys().cloned().collect::<Vec<_>>();

        for station in self.stations.values() {
            for commit in station.commits() {
                match send_request(
                    station.address,
                    Some(commit.file_uuid),
                    from_station_uuid.clone(),
                )
                .await
                {
                    Ok(Some(bytes)) => {
                        files.push(RequestedFile {
                            uuid: commit.file_uuid,
                            name: commit
                                .file_name
                                .clone()
                                .unwrap_or(format!("{}.mvr", commit.file_uuid)),
                            bytes,
                        });
                    }
                    Ok(None) => {}
                    Err(err) => {
                        log::warn!("Failed to send request to station {}: {}", station.uuid(), err);
                    }
                }
            }
        }
        Ok(files)
    }

    fn local_commits(&self) -> Vec<Commit> {
        self.local_mvr_files
            .values()
            .map(|mvr_file| {
                Commit {
                    file_uuid: mvr_file.file_hash_uuid(),
                    station_uuid: self.settings.station_uuid,
                    file_size: mvr_file.file_size(),
                    ver_major: mvr_file.general_scene_description().ver_major,
                    ver_minor: mvr_file.general_scene_description().ver_major,
                    // FIXME: Properly handle `ForStationUUID`. Right now we assume they
                    // should be sent to all stations in the network.
                    for_stations_uuid: Vec::new(),
                    file_name: Some(mvr_file.file_name().to_string()),
                    // If the commit needs a comment, it should be manually added before sending it.
                    comment: None,
                }
            })
            .collect()
    }

    fn local_commit(&self, file_uuid: &Uuid) -> Option<Commit> {
        self.local_mvr_files.get(file_uuid).map(|mvr_file| Commit {
            file_uuid: mvr_file.file_hash_uuid(),
            station_uuid: self.settings.station_uuid,
            file_size: mvr_file.file_size(),
            ver_major: mvr_file.general_scene_description().ver_major,
            ver_minor: mvr_file.general_scene_description().ver_major,
            // FIXME: Properly handle `ForStationUUID`. Right now we assume they
            // should be sent to all stations in the network.
            for_stations_uuid: Vec::new(),
            file_name: Some(mvr_file.file_name().to_string()),
            // If the commit needs a comment, it should be manually added before sending it.
            comment: None,
        })
    }

    fn register_station(&mut self, info: StationInfo) {
        let uuid = info.uuid();

        // If it existed as inactive, activate it.
        let _ = self.inactive_stations.remove(&uuid);

        if self.stations.insert(uuid, info.clone()).is_none() {
            self.emit_on_join(info);
        }
    }

    fn unregister_station(&mut self, uuid: &Uuid) {
        if let Some(info) = self.stations.remove(uuid) {
            log::info!("Station {} ({}) unregistered", info.name(), info.uuid());

            // Keep it around as inactive so mDNS rediscovery won't trigger an immediate re-join,
            // while preserving station metadata for the user.
            self.inactive_stations.insert(info.uuid(), info.clone());

            self.emit_on_leave(info);
        }
    }

    fn emit_on_join(&self, info: StationInfo) {
        if let Some(handler) = &self.on_join {
            handler(info);
        }
    }

    fn emit_on_leave(&self, info: StationInfo) {
        if let Some(handler) = &self.on_leave {
            handler(info);
        }
    }
}

async fn handle_inbound(
    conn: TcpStream,
    addr: SocketAddr,
    sender: mpsc::Sender<InternalEvent>,
) -> Result<(), Error> {
    let mut framed = Framed::new(conn, PacketCodec);

    let packet = framed.next().await.ok_or(Error::ConnectionClosed)??;

    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(InternalEvent::InboundPacket { payload: packet.payload, addr, resp: resp_tx })
        .await
        .map_err(|_| Error::Shutdown)?;

    let ret_payload = resp_rx.await.map_err(|_| Error::Shutdown)??;
    framed.send(Packet::new(ret_payload, 0, 1)?).await?;

    Ok(())
}

async fn send_leave(addr: SocketAddr, from_station_uuid: Uuid) -> Result<(), Error> {
    let packet = Packet::new(PacketPayload::Leave { from_station_uuid }, 0, 1)?;
    match send_packet_and_recv(addr, packet).await?.payload {
        PacketPayload::LeaveRet { ok: true, .. } => Ok(()),
        PacketPayload::LeaveRet { ok: false, message, .. } => Err(Error::InvalidPacket { message }),
        _ => Err(Error::UnexpectedPacket),
    }
}

async fn send_commit(addr: SocketAddr, commit: Commit) -> Result<(), Error> {
    let packet = Packet::new(
        PacketPayload::Commit {
            file_uuid: commit.file_uuid,
            station_uuid: commit.station_uuid,
            file_size: commit.file_size,
            ver_major: commit.ver_major,
            ver_minor: commit.ver_minor,
            for_stations_uuid: commit.for_stations_uuid,
            file_name: commit.file_name,
            comment: commit.comment,
        },
        0,
        1,
    )?;
    match send_packet_and_recv(addr, packet).await?.payload {
        PacketPayload::CommitRet { ok: true, .. } => Ok(()),
        PacketPayload::CommitRet { ok: false, message, .. } => {
            Err(Error::InvalidPacket { message })
        }
        _ => Err(Error::UnexpectedPacket),
    }
}

async fn send_request(
    addr: SocketAddr,
    file_uuid: Option<Uuid>,
    from_station_uuid: Vec<Uuid>,
) -> Result<Option<Vec<u8>>, Error> {
    let packet = Packet::new(PacketPayload::Request { file_uuid, from_station_uuid }, 0, 1)?;
    match send_packet_and_recv(addr, packet).await?.payload {
        PacketPayload::File(bytes) => Ok(Some(bytes)),
        PacketPayload::RequestRet { ok: true, .. } => Err(Error::UnexpectedPacket),
        PacketPayload::RequestRet { ok: false, message, .. } => {
            Err(Error::InvalidPacket { message })
        }
        _ => Err(Error::UnexpectedPacket),
    }
}

async fn send_packet_and_recv(socket_addr: SocketAddr, packet: Packet) -> Result<Packet, Error> {
    let stream = time::timeout(Duration::from_secs(1), TcpStream::connect(socket_addr))
        .await
        .map_err(|_| Error::Timeout)??;

    let mut framed = Framed::new(stream, PacketCodec);
    framed.send(packet).await?;
    framed.next().await.ok_or(Error::ConnectionClosed)?
}

/// A file that has been requested using `MVR_REQUEST`.
pub struct RequestedFile {
    uuid: Uuid,
    name: String,
    bytes: Vec<u8>,
}

impl RequestedFile {
    /// The file's UUID.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// The file's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The raw bytes of the compressed file.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
