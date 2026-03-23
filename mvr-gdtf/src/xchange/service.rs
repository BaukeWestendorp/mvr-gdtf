use std::{
    collections::HashMap,
    io,
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures_util::{SinkExt as _, StreamExt as _};
use mdns_sd::{ResolvedService, ServiceEvent, ServiceInfo};
use tokio::{
    fs,
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc},
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

/// Everything needed to load an MVR file into the station pool.
#[derive(Debug, Clone)]
pub struct MvrSource {
    pub(crate) bytes: Bytes,
    pub(crate) name: Option<String>,
}

impl MvrSource {
    /// Reads the file at `path` asynchronously.
    ///
    /// # Errors
    /// Returns an [`io::Error`] if the file cannot be read.
    pub async fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let name = path.file_name().and_then(|n| n.to_str()).map(ToString::to_string);
        let bytes = fs::read(path).await?.into();
        Ok(Self { bytes, name })
    }

    /// Creates a source from raw bytes without an attached name.
    ///
    /// Call [`.named()`](MvrSource::named) to attach a name.
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self { bytes: bytes.into(), name: None }
    }

    /// Reads all bytes from `reader` without an attached name.
    ///
    /// Call [`.named()`](MvrSource::named) to attach a name.
    ///
    /// # Errors
    /// Returns an [`io::Error`] if reading fails.
    pub fn from_reader(mut reader: impl io::Read) -> io::Result<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(Self { bytes: buf.into(), name: None })
    }

    /// Attaches or replaces the file name, consuming and returning `self`.
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl From<Vec<u8>> for MvrSource {
    fn from(v: Vec<u8>) -> Self {
        Self::from_bytes(v)
    }
}

impl From<bytes::Bytes> for MvrSource {
    fn from(b: bytes::Bytes) -> Self {
        Self::from_bytes(b)
    }
}

impl From<&[u8]> for MvrSource {
    fn from(s: &[u8]) -> Self {
        Self::from_bytes(Bytes::copy_from_slice(s))
    }
}

/// A builder for configuring and starting the MVR-xchange service.
#[derive(Debug, Clone)]
pub struct StationBuilder {
    provider_name: String,
    station_name: String,
    group_name: String,
    station_uuid: Uuid,
    interface_ip: Option<IpAddr>,
    port: u16,
}

impl StationBuilder {
    /// Create a new builder with required base parameters.
    pub fn new(provider_name: impl Into<String>, station_name: impl Into<String>) -> Self {
        Self {
            provider_name: provider_name.into(),
            station_name: station_name.into(),
            group_name: "Default".to_string(),
            station_uuid: Uuid::new_v4(),
            interface_ip: None,
            port: 48484,
        }
    }

    /// Set the group name of the service we want to connect to.
    pub fn group_name(mut self, group: impl Into<String>) -> Self {
        self.group_name = group.into();
        self
    }

    /// Set the UUID for this station.
    pub fn station_uuid(mut self, uuid: Uuid) -> Self {
        self.station_uuid = uuid;
        self
    }

    /// The IP of the interface we want to listen on.
    pub fn interface_ip(mut self, ip: IpAddr) -> Self {
        self.interface_ip = Some(ip);
        self
    }

    /// The port we want to listen on.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Builds the configuration and starts the background service.
    pub fn start(self) -> Result<Service, Error> {
        let interface_ip = self
            .interface_ip
            .unwrap_or_else(|| local_ip_address::local_ip().expect("Should get local IP"));

        let settings = Settings {
            provider_name: self.provider_name,
            group_name: self.group_name,
            // Sanitize station name
            station_name: self
                .station_name
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
                .collect(),
            station_uuid: self.station_uuid,
            interface_ip,
            port: self.port,
        };

        Service::start(settings)
    }
}

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

/// Events emitted by the MVR-xchange network.
#[derive(Debug, Clone)]
pub enum StationEvent {
    /// Another station joined the network.
    Joined(StationInfo),
    /// A station left the network.
    Left(StationInfo),
    /// A station committed a new MVR file.
    Committed {
        /// The station that committed the file.
        station: StationInfo,
        /// The UUID of the file that was committed.
        file_uuid: Uuid,
    },
    /// A station requested one of our MVR files.
    Requested {
        /// The station that requested the MVR file.
        station: StationInfo,
        /// The UUID of the MVR file the other station requested.
        /// `None` if it requested the latest MVR file.
        file_uuid: Option<Uuid>,
    },
}

enum Command {
    LoadParsedMvrFile {
        source: MvrSource,
        mvr_file: MvrFile,
        resp: flume::Sender<Result<Uuid, Error>>,
    },
    GetLocalMvrFiles {
        resp: flume::Sender<Vec<Arc<MvrFile>>>,
    },
    GetLocalMvrFile {
        file_uuid: Uuid,
        resp: flume::Sender<Option<Arc<MvrFile>>>,
    },
    UnloadLocalMvrFile {
        file_uuid: Uuid,
        resp: flume::Sender<Result<(), Error>>,
    },
    Join {
        target_uuid: Uuid,
        resp: flume::Sender<Result<(), Error>>,
    },
    JoinAll {
        resp: flume::Sender<Result<(), Error>>,
    },
    Leave {
        target_uuid: Uuid,
        resp: flume::Sender<Result<(), Error>>,
    },
    LeaveAll {
        resp: flume::Sender<Result<(), Error>>,
    },
    Commit {
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
        resp: flume::Sender<Result<(), Error>>,
    },
    CommitAll {
        file_uuid: Uuid,
        comment: Option<String>,
        resp: flume::Sender<Result<(), Error>>,
    },
    Request {
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
        resp: flume::Sender<Result<Option<RequestedFile>, Error>>,
    },
    RequestAll {
        resp: flume::Sender<Result<Vec<RequestedFile>, Error>>,
    },
    Stations {
        resp: flume::Sender<Vec<StationInfo>>,
    },
    Shutdown {
        resp: flume::Sender<()>,
    },
}

enum InternalEvent {
    InboundPacket {
        payload: PacketPayload,
        resp: flume::Sender<Result<Option<PacketPayload>, Error>>,
        from_station_uuid: Uuid,
    },
}

/// A cloneable handle to a running MVR-xchange station service.
#[derive(Clone)]
pub struct Service {
    sender: mpsc::Sender<Command>,
    event_tx: broadcast::Sender<StationEvent>,
}

impl Service {
    fn start(settings: Settings) -> Result<Self, Error> {
        let (sender, receiver) = mpsc::channel(32);
        let (event_tx, _) = broadcast::channel(128);

        tokio::spawn({
            let event_tx = event_tx.clone();
            async move {
                if let Err(err) = Inner::new(settings, event_tx).run(receiver).await {
                    log::warn!("{err}");
                }
            }
        });

        Ok(Self { sender, event_tx })
    }

    /// Subscribe to network events (joins, leaves, commits, requests).
    pub fn subscribe(&self) -> broadcast::Receiver<StationEvent> {
        self.event_tx.subscribe()
    }

    async fn send_cmd(&self, cmd: Command) -> Result<(), Error> {
        self.sender.send(cmd).await.map_err(|_| Error::Shutdown)
    }

    fn blocking_recv<T>(rx: flume::Receiver<T>) -> Result<T, Error> {
        rx.recv().map_err(|_| Error::Shutdown)
    }

    fn blocking_recv_result<T>(rx: flume::Receiver<Result<T, Error>>) -> Result<T, Error> {
        Self::blocking_recv(rx)?
    }

    fn blocking_send_cmd(&self, cmd: Command) -> Result<(), Error> {
        self.sender.blocking_send(cmd).map_err(|_| Error::Shutdown)
    }

    /// Loads an MVR file (parsing is offloaded via `spawn_blocking`) and returns its UUID.
    ///
    /// # Errors
    /// Returns [`Error::InvalidMvrFileBytes`] if the bytes are not a valid MVR
    /// file, [`Error::DuplicateLocalMvrFile`] if a file with the same content
    /// hash is already in the pool, or [`Error::Shutdown`] if the background
    /// task has stopped.
    pub async fn load_mvr_file_async(&self, source: impl Into<MvrSource>) -> Result<Uuid, Error> {
        let source = source.into();
        let (tx, rx) = flume::unbounded();

        let source_clone = source.clone();
        let mvr_file =
            tokio::task::spawn_blocking(move || MvrFile::load_from_bytes(&source_clone.bytes))
                .await
                .map_err(|_| Error::Shutdown)?
                .map_err(|_| Error::InvalidMvrFileBytes)?;

        self.send_cmd(Command::LoadParsedMvrFile { source, mvr_file, resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::load_mvr_file_async`].
    ///
    /// This must be called from a context where it's OK to block the current
    /// thread. It does not require an async runtime.
    pub fn load_mvr_file(&self, source: impl Into<MvrSource>) -> Result<Uuid, Error> {
        let source = source.into();
        let (tx, rx) = flume::unbounded();

        let source_clone = source.clone();
        let mvr_file = MvrFile::load_from_bytes(&source_clone.bytes)
            .map_err(|_| Error::InvalidMvrFileBytes)?;

        self.blocking_send_cmd(Command::LoadParsedMvrFile { source, mvr_file, resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Returns all locally stored MVR files.
    ///
    /// # Errors
    /// Returns [`Error::Shutdown`] if the background task has stopped.
    pub async fn local_mvr_files_async(&self) -> Result<Vec<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::GetLocalMvrFiles { resp: tx }).await?;
        Ok(rx.recv_async().await.map_err(|_| Error::Shutdown)?)
    }

    /// Blocking version of [`Service::local_mvr_files_async`].
    pub fn local_mvr_files(&self) -> Result<Vec<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::GetLocalMvrFiles { resp: tx })?;
        Ok(Self::blocking_recv(rx)?)
    }

    /// Returns the locally stored MVR file identified by `file_uuid`, or
    /// `None` if the file was not found.
    ///
    /// # Errors
    /// Returns [`Error::Shutdown`] if the background task has stopped.
    pub async fn local_mvr_file_async(
        &self,
        file_uuid: Uuid,
    ) -> Result<Option<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::GetLocalMvrFile { file_uuid, resp: tx }).await?;
        Ok(rx.recv_async().await.map_err(|_| Error::Shutdown)?)
    }

    /// Blocking version of [`Service::local_mvr_file_async`].
    pub fn local_mvr_file(&self, file_uuid: Uuid) -> Result<Option<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::GetLocalMvrFile { file_uuid, resp: tx })?;
        Ok(Self::blocking_recv(rx)?)
    }

    /// Removes the locally stored MVR file identified by `file_uuid` from the
    /// pool. The file will no longer be advertised to peers or returned in
    /// response to [`Service::request`].
    ///
    /// # Errors
    /// Returns [`Error::LocalMvrFileNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn unload_local_mvr_file_async(&self, file_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::UnloadLocalMvrFile { file_uuid, resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::unload_local_mvr_file_async`].
    pub fn unload_local_mvr_file(&self, file_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::UnloadLocalMvrFile { file_uuid, resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_JOIN` to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn join_async(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::Join { target_uuid, resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::join_async`].
    pub fn join(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::Join { target_uuid, resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_JOIN` to every currently known station.
    ///
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn join_all_async(&self) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::JoinAll { resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::join_all_async`].
    pub fn join_all(&self) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::JoinAll { resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_LEAVE` to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn leave_async(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::Leave { target_uuid, resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::leave_async`].
    pub fn leave(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::Leave { target_uuid, resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_LEAVE` to every currently known station.
    ///
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn leave_all_async(&self) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::LeaveAll { resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::leave_all_async`].
    pub fn leave_all(&self) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::LeaveAll { resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_COMMIT` for the MVR file associated with `file_uuid`
    /// to the station identified by `target_uuid`.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if the station UUID is unknown,
    /// [`Error::LocalMvrFileNotFound`] if the file UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn commit_async(
        &self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::Commit {
            target_uuid,
            file_uuid,
            comment: comment.map(Into::into),
            resp: tx,
        })
        .await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::commit_async`].
    pub fn commit(
        &self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::Commit {
            target_uuid,
            file_uuid,
            comment: comment.map(Into::into),
            resp: tx,
        })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_COMMIT` for the MVR file associated with `file_uuid`
    /// to every currently known station.
    ///
    /// Per-station errors are logged and skipped; the call only returns
    /// [`Error::LocalMvrFileNotFound`] if the file UUID is unknown, or
    /// [`Error::Shutdown`] if the background task has stopped.
    pub async fn commit_all_async(
        &self,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::CommitAll { file_uuid, comment: comment.map(Into::into), resp: tx })
            .await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::commit_all_async`].
    pub fn commit_all(
        &self,
        file_uuid: Uuid,
        comment: Option<impl Into<String>>,
    ) -> Result<(), Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::CommitAll {
            file_uuid,
            comment: comment.map(Into::into),
            resp: tx,
        })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_REQUEST` to request the file identified by `file_uuid` from
    /// the given stations.
    ///
    /// Stations are tried in order; the first one that returns a file wins.
    /// Returns [`None`] if no matching file was found on any of the targeted
    /// stations.
    ///
    /// # Errors
    /// Returns [`Error::StationNotFound`] if any UUID in `from_station_uuid` is
    /// unknown, or [`Error::Shutdown`] if the background task has stopped.
    pub async fn request_async(
        &self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
    ) -> Result<Option<RequestedFile>, Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::Request { file_uuid, from_station_uuid, resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::request_async`].
    pub fn request(
        &self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
    ) -> Result<Option<RequestedFile>, Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::Request { file_uuid, from_station_uuid, resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Sends `MVR_REQUEST` to every currently known station and collects all
    /// returned files.
    ///
    /// Stations that fail or return no file are logged and skipped. The call
    /// only returns [`Error::Shutdown`] if the background task has stopped.
    pub async fn request_all_async(&self) -> Result<Vec<RequestedFile>, Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::RequestAll { resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)?
    }

    /// Blocking version of [`Service::request_all_async`].
    pub fn request_all(&self) -> Result<Vec<RequestedFile>, Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::RequestAll { resp: tx })?;
        Self::blocking_recv_result(rx)
    }

    /// Returns a snapshot of all currently known stations.
    ///
    /// # Errors
    /// Returns [`Error::Shutdown`] if the background task has stopped.
    pub async fn stations_async(&self) -> Result<Vec<StationInfo>, Error> {
        let (tx, rx) = flume::unbounded();
        self.send_cmd(Command::Stations { resp: tx }).await?;
        rx.recv_async().await.map_err(|_| Error::Shutdown)
    }

    /// Blocking version of [`Service::stations_async`].
    pub fn stations(&self) -> Result<Vec<StationInfo>, Error> {
        let (tx, rx) = flume::unbounded();
        self.blocking_send_cmd(Command::Stations { resp: tx })?;
        Ok(Self::blocking_recv(rx)?)
    }

    /// Gracefully shuts down the background task.
    ///
    /// Sends `MVR_LEAVE` to all known stations, then waits for the task to
    /// finish. Dropping all [`Service`] handles also triggers a shutdown, but
    /// without the ability to await its completion.
    pub async fn shutdown_async(&self) {
        let (tx, rx) = flume::unbounded();
        if self.send_cmd(Command::Shutdown { resp: tx }).await.is_ok() {
            let _ = rx.recv_async().await;
        }
    }

    /// Blocking version of [`Service::shutdown_async`].
    ///
    /// Sends `MVR_LEAVE` to all known stations, then waits for the task to
    /// finish. Dropping all [`Service`] handles also triggers a shutdown, but
    /// without the ability to wait for its completion.
    pub fn shutdown(&self) {
        let (tx, rx) = flume::unbounded();
        if self.blocking_send_cmd(Command::Shutdown { resp: tx }).is_ok() {
            let _ = rx.recv();
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
    local_mvr_files: HashMap<Uuid, LocalMvrFile>,
    latest_local_mvr_file: Option<Uuid>,

    event_tx: broadcast::Sender<StationEvent>,
}

impl Inner {
    fn new(settings: Settings, event_tx: broadcast::Sender<StationEvent>) -> Self {
        // Sanitize station name
        let mut settings = settings;
        settings.station_name = settings
            .station_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
            .collect();

        Self {
            settings,
            stations: HashMap::new(),
            inactive_stations: HashMap::new(),
            local_mvr_files: HashMap::new(),
            latest_local_mvr_file: None,
            event_tx,
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
                    if let Some(InternalEvent::InboundPacket { payload, resp, from_station_uuid }) = event {
                        let _ = resp.send(self.handle_inbound_packet(payload, from_station_uuid));
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
            Command::LoadParsedMvrFile { source, mvr_file, resp } => {
                let uuid = mvr_file.file_hash_uuid();

                if self.local_mvr_files.contains_key(&uuid) {
                    let _ = resp.send(Err(Error::DuplicateLocalMvrFile { uuid }));
                    return;
                }

                self.latest_local_mvr_file = Some(uuid);
                self.local_mvr_files.insert(
                    uuid,
                    LocalMvrFile {
                        mvr_file: Arc::new(mvr_file),
                        bytes: source.bytes,
                        name: source.name,
                    },
                );

                let _ = resp.send(Ok(uuid));
            }
            Command::GetLocalMvrFiles { resp } => {
                let files =
                    self.local_mvr_files.values().map(|f| Arc::clone(&f.mvr_file)).collect();
                let _ = resp.send(files);
            }
            Command::GetLocalMvrFile { file_uuid, resp } => {
                let file = self.local_mvr_files.get(&file_uuid).map(|f| Arc::clone(&f.mvr_file));
                let _ = resp.send(file);
            }
            Command::UnloadLocalMvrFile { file_uuid, resp } => {
                if self.local_mvr_files.remove(&file_uuid).is_some() {
                    if self.latest_local_mvr_file == Some(file_uuid) {
                        self.latest_local_mvr_file = self.local_mvr_files.keys().next().copied();
                    }
                    let _ = resp.send(Ok(()));
                } else {
                    let _ = resp.send(Err(Error::LocalMvrFileNotFound { uuid: file_uuid }));
                }
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
            Command::Shutdown { .. } => unreachable!("Shutdown is handled in run()"),
        }
    }

    fn handle_inbound_packet(
        &mut self,
        payload: PacketPayload,
        from_station_uuid: Uuid,
    ) -> Result<Option<PacketPayload>, Error> {
        let ret = match payload {
            PacketPayload::Join {
                provider,
                station_name,
                station_uuid,
                ver_major,
                ver_minor,
                commits,
            } => {
                // `addr` is the TCP peer/source address and its port is typically ephemeral.
                // We must NOT treat it as the station's listener port (which is discovered via mDNS).
                if let Some(existing) = self.stations.get_mut(&station_uuid) {
                    existing.last_seen = Instant::now();
                    existing.commits = commits;
                } else if let Some(existing) = self.inactive_stations.get(&station_uuid).cloned() {
                    // We know the listener address (from mDNS) because it exists in inactive_stations.
                    // Activate it now: this will emit `on_join`.
                    self.register_station(StationInfo::new(
                        station_uuid,
                        station_name,
                        existing.address,
                        provider,
                        ver_major,
                        ver_minor,
                        commits,
                    ));
                } else {
                    // Unknown station and no mDNS address yet: ignore for now.
                    // We'll activate it when mDNS resolves its service (see handle_resolved_mdns_service).
                    log::debug!(
                        "Received JOIN from station {station_uuid} but no mDNS address is known yet; delaying activation"
                    );
                }

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

                if let Some(station) = self.stations.get(&station_uuid) {
                    let _ = self
                        .event_tx
                        .send(StationEvent::Committed { station: station.clone(), file_uuid });
                }

                PacketPayload::CommitRet { ok: true, message: String::new() }
            }

            PacketPayload::Request { file_uuid, from_station_uuid: station_filter } => {
                if !station_filter.is_empty()
                    && !station_filter.contains(&self.settings.station_uuid)
                {
                    return Ok(None);
                }

                let local_mvr_file = match file_uuid {
                    Some(file_uuid) => self.local_mvr_files.get(&file_uuid),
                    None => self
                        .latest_local_mvr_file
                        .as_ref()
                        .and_then(|uuid| self.local_mvr_files.get(uuid)),
                };

                match local_mvr_file {
                    Some(f) => {
                        if let Some(station) = self.stations.get(&from_station_uuid) {
                            let _ = self.event_tx.send(StationEvent::Requested {
                                station: station.clone(),
                                file_uuid,
                            });
                        }

                        PacketPayload::File(Bytes::clone(&f.bytes))
                    }
                    None => PacketPayload::RequestRet {
                        ok: false,
                        message: "The MVR is not available on this client".to_string(),
                    },
                }
            }
            PacketPayload::NewSessionHost { .. } => todo!("handle MVR_NEW_SESSION_HOST"),
            _ => return Err(Error::UnexpectedPacket),
        };

        Ok(Some(ret))
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

        // If already known & active, just refresh the timestamp.
        if let Some(station) = self.stations.get_mut(&uuid) {
            station.last_seen = Instant::now();
            return;
        }

        // If already known but inactive, don't auto-join.
        // Still update the stored address so a manual join uses the latest endpoint.
        if let Some(station) = self.inactive_stations.get_mut(&uuid) {
            if let Some(addr) = service.get_addresses().iter().next() {
                station.address = SocketAddr::from((addr.to_ip_addr(), service.get_port()));
            }
            return;
        }

        // Unknown station: resolve gives us the listener port. Handshake now; on success we
        // register+emit `on_join`.
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
            return Err(Error::LocalMvrFileNotFound { uuid: file_uuid });
        };
        commit.comment = comment;
        send_commit(addr, commit).await
    }

    async fn do_commit_all(
        &mut self,
        file_uuid: Uuid,
        comment: Option<String>,
    ) -> Result<(), Error> {
        let stations: Vec<(Uuid, SocketAddr)> =
            self.stations.values().map(|s| (s.uuid(), s.address)).collect();
        let Some(mut commit) = self.local_commit(&file_uuid) else {
            return Err(Error::LocalMvrFileNotFound { uuid: file_uuid });
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
                let name = self
                    .stations
                    .get(&station_uuid)
                    .and_then(|s| s.commits().iter().find_map(|c| c.file_name.clone()))
                    .unwrap_or_else(|| format!("{}.mvr", file_uuid.unwrap_or_default()));

                return Ok(Some(RequestedFile { uuid: station_uuid, name, bytes: bytes.into() }));
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
                                .unwrap_or_else(|| format!("{}.mvr", commit.file_uuid)),
                            bytes: bytes.into(),
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
            .map(|f| {
                let mvr = &f.mvr_file;
                Commit {
                    file_uuid: mvr.file_hash_uuid(),
                    station_uuid: self.settings.station_uuid,
                    file_size: f.bytes.len() as u64,
                    ver_major: mvr.general_scene_description().ver_major,
                    ver_minor: mvr.general_scene_description().ver_minor,
                    // FIXME: Properly handle `ForStationUUID`. Right now we assume they
                    // should be sent to all stations in the network.
                    for_stations_uuid: Vec::new(),
                    file_name: f.name.clone(),
                    // If the commit needs a comment, it should be manually added before sending it.
                    comment: None,
                }
            })
            .collect()
    }

    fn local_commit(&self, file_uuid: &Uuid) -> Option<Commit> {
        self.local_mvr_files.get(file_uuid).map(|f| {
            let mvr = &f.mvr_file;
            Commit {
                file_uuid: mvr.file_hash_uuid(),
                station_uuid: self.settings.station_uuid,
                file_size: f.bytes.len() as u64,
                ver_major: mvr.general_scene_description().ver_major,
                ver_minor: mvr.general_scene_description().ver_minor,
                // FIXME: Properly handle `ForStationUUID`. Right now we assume they
                // should be sent to all stations in the network.
                for_stations_uuid: Vec::new(),
                file_name: f.name.clone(),
                // If the commit needs a comment, it should be manually added before sending it.
                comment: None,
            }
        })
    }

    fn register_station(&mut self, info: StationInfo) {
        let uuid = info.uuid();

        // If it existed as inactive, activate it.
        let _ = self.inactive_stations.remove(&uuid);
        if self.stations.insert(uuid, info.clone()).is_none() {
            let _ = self.event_tx.send(StationEvent::Joined(info));
        }
    }

    fn unregister_station(&mut self, uuid: &Uuid) {
        if let Some(info) = self.stations.remove(uuid) {
            log::info!("Station {} ({}) unregistered", info.name(), info.uuid());
            self.inactive_stations.insert(info.uuid(), info.clone());
            let _ = self.event_tx.send(StationEvent::Left(info));
        }
    }
}

async fn handle_inbound(
    conn: TcpStream,
    peer_addr: SocketAddr,
    sender: mpsc::Sender<InternalEvent>,
) -> Result<(), Error> {
    let mut framed = Framed::new(conn, PacketCodec);

    let packet = framed.next().await.ok_or(Error::ConnectionClosed)??;

    // Derive the station UUID from the packet payload instead of trying to map by peer socket.
    // If we cannot derive it, fall back to Nil (still lets us respond to requests).
    let derived_station_uuid = match &packet.payload {
        PacketPayload::Join { station_uuid, .. } => *station_uuid,
        PacketPayload::Leave { from_station_uuid } => *from_station_uuid,
        PacketPayload::Commit { station_uuid, .. } => *station_uuid,
        // For REQUEST, the payload does not include a single sender UUID. We'll log it and
        // continue with nil;
        PacketPayload::Request { .. } => Uuid::nil(),
        _ => Uuid::nil(),
    };

    let (resp_tx, resp_rx) = flume::unbounded();
    sender
        .send(InternalEvent::InboundPacket {
            payload: packet.payload,
            resp: resp_tx,
            from_station_uuid: derived_station_uuid,
        })
        .await
        .map_err(|_| Error::Shutdown)?;

    if let Some(ret_payload) = resp_rx.recv_async().await.map_err(|_| Error::Shutdown)?? {
        log::trace!(">>> Sending return packet to peer_addr={peer_addr}: {:?}", ret_payload);
        framed.send(Packet::new(ret_payload, 0, 1)?).await?;
    }

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
) -> Result<Option<Bytes>, Error> {
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
    log::trace!(">>> Sending packet: {:?}", packet.payload);
    framed.send(packet).await?;
    let ret_packet = framed.next().await.ok_or(Error::ConnectionClosed)??;
    log::trace!("<<< Received return packet: {:?}", ret_packet.payload);
    Ok(ret_packet)
}

/// A file received in response to an `MVR_REQUEST`.
pub struct RequestedFile {
    uuid: Uuid,
    name: String,
    bytes: Bytes,
}

impl RequestedFile {
    /// The UUID identifying this file (either the requested file UUID, or the
    /// UUID of the station that provided it when no specific file was
    /// requested).
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// The file name, derived from the remote station's commit metadata.
    /// Falls back to `"<uuid>.mvr"` if no name was advertised.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The raw bytes of the compressed MVR file.
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }
}

struct LocalMvrFile {
    mvr_file: Arc<MvrFile>,
    bytes: bytes::Bytes,
    name: Option<String>,
}
