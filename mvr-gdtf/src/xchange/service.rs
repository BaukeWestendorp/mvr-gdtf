use std::{
    collections::HashMap,
    fs,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures_util::{SinkExt as _, StreamExt as _};
use mdns_sd::{ResolvedService, ServiceEvent, ServiceInfo};
use tokio::{
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
        packet::{Packet, PacketCodec, PacketPayload},
    },
};

/// The mDNS service type used to register and discover MVR-xchange stations.
pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";

const REGISTRATION_INTERVAL: Duration = Duration::from_secs(10);
const STALE_THRESHOLD: Duration = Duration::from_secs(30);
const PURGE_INTERVAL: Duration = Duration::from_secs(5);

const SUPPORTED_MVR_FILE_VERSION_MAJOR: u32 = 1;
const SUPPORTED_MVR_FILE_VERSION_MINOR: u32 = 6;

pub enum MvrSource {
    Path { path: PathBuf },
    Bytes { bytes: Bytes, file_name: Option<String> },
}

impl From<Vec<u8>> for MvrSource {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes { bytes: value.into(), file_name: None }
    }
}

impl From<Bytes> for MvrSource {
    fn from(value: Bytes) -> Self {
        Self::Bytes { bytes: value, file_name: None }
    }
}

impl From<&[u8]> for MvrSource {
    fn from(value: &[u8]) -> Self {
        Self::Bytes { bytes: value.to_vec().into(), file_name: None }
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

    pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
        self.group_name = group_name.into();
        self
    }

    pub fn station_uuid(mut self, station_uuid: Uuid) -> Self {
        self.station_uuid = station_uuid;
        self
    }

    pub fn interface_ip(mut self, interface_ip: IpAddr) -> Self {
        self.interface_ip = Some(interface_ip);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn start(self) -> Result<Service, Error> {
        let interface_ip = match self.interface_ip {
            Some(ip) => ip,
            None => local_ip_address::local_ip().map_err(|_| Error::Shutdown)?,
        };

        let settings = Settings {
            provider_name: self.provider_name,
            group_name: self.group_name,
            station_name: self.station_name,
            station_uuid: self.station_uuid,
            interface_ip,
            port: self.port,
        };

        let (cmd_tx, cmd_rx) = mpsc::channel(100);
        let (event_tx, _) = broadcast::channel(100);

        let service = Service { tx: cmd_tx, events: event_tx.clone() };

        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build();

            let Ok(rt) = rt else {
                log::error!("Failed to build MVR runtime");
                return;
            };

            rt.block_on(async move {
                let mut actor = StationActor::new(settings, cmd_rx, event_tx);
                if let Err(e) = actor.run().await {
                    log::error!("MVR actor exited with error: {:?}", e);
                }
            });
        });

        Ok(service)
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub provider_name: String,
    pub group_name: String,
    pub station_name: String,
    pub station_uuid: Uuid,
    pub interface_ip: IpAddr,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider_name: "MVR GDTF".to_string(),
            group_name: "Default".to_string(),
            station_name: "Station".to_string(),
            station_uuid: Uuid::new_v4(),
            interface_ip: IpAddr::from([127, 0, 0, 1]),
            port: 48484,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StationEvent {
    Joined(StationInfo),
    Left(StationInfo),
    Committed { station: StationInfo, file_uuid: Uuid },
    Requested { station: StationInfo, file_uuid: Option<Uuid> },
}

/// Internal commands sent from the Sync API to the Async Actor.
enum ActorMessage {
    InsertLocalFile {
        file: MvrFile,
        bytes: Bytes,
        resp: flume::Sender<Result<Uuid, Error>>,
    },
    GetLocalFiles {
        resp: flume::Sender<Vec<Arc<MvrFile>>>,
    },
    GetLocalFile {
        file_uuid: Uuid,
        resp: flume::Sender<Option<Arc<MvrFile>>>,
    },
    UnloadLocalFile {
        file_uuid: Uuid,
        resp: flume::Sender<Result<(), Error>>,
    },
    Join {
        target_uuid: Uuid,
        resp: flume::Sender<Result<(), Error>>,
    },
    Commit {
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
        resp: flume::Sender<Result<(), Error>>,
    },
    Request {
        file_uuid: Option<Uuid>,
        from_station_uuid: Uuid,
        resp: flume::Sender<Result<Option<RequestedFile>, Error>>,
    },
    GetStations {
        resp: flume::Sender<Vec<StationInfo>>,
    },
    Shutdown {
        resp: flume::Sender<()>,
    },
}

/// A cloneable handle to the MVR-xchange service with a synchronous API.
#[derive(Clone)]
pub struct Service {
    tx: mpsc::Sender<ActorMessage>,
    events: broadcast::Sender<StationEvent>,
}

impl Service {
    fn call<T>(&self, msg: ActorMessage, rx: flume::Receiver<T>) -> Result<T, Error> {
        self.tx.blocking_send(msg).map_err(|_| Error::Shutdown)?;
        rx.recv().map_err(|_| Error::Shutdown)
    }

    pub fn subscribe(&self) -> flume::Receiver<StationEvent> {
        let (tx, rx) = flume::unbounded();
        let mut b_rx = self.events.subscribe();

        thread::spawn(move || {
            while let Ok(event) = b_rx.blocking_recv() {
                if tx.send(event).is_err() {
                    break;
                }
            }
        });

        rx
    }

    pub fn add_mvr_file(&self, source: impl Into<MvrSource>) -> Result<Uuid, Error> {
        let (bytes, file_name) = match source.into() {
            MvrSource::Path { path } => {
                let bytes: Bytes = fs::read(&path).map_err(|_| Error::InvalidMvrFileBytes)?.into();
                let file_name = path.file_name().map(|s| s.to_string_lossy().to_string());
                (bytes, file_name)
            }
            MvrSource::Bytes { bytes, file_name } => (bytes, file_name),
        };

        let file =
            MvrFile::load_from_bytes(&bytes, file_name).map_err(|_| Error::InvalidMvrFileBytes)?;
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::InsertLocalFile { file, bytes, resp: tx }, rx)?
    }

    pub fn local_mvr_files(&self) -> Result<Vec<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::GetLocalFiles { resp: tx }, rx)
    }

    pub fn local_mvr_file(&self, file_uuid: Uuid) -> Result<Option<Arc<MvrFile>>, Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::GetLocalFile { file_uuid, resp: tx }, rx)
    }

    pub fn unload_local_mvr_file(&self, file_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::UnloadLocalFile { file_uuid, resp: tx }, rx)?
    }

    pub fn join(&self, target_uuid: Uuid) -> Result<(), Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::Join { target_uuid, resp: tx }, rx)?
    }

    pub fn commit(
        &self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
    ) -> Result<(), Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::Commit { target_uuid, file_uuid, comment, resp: tx }, rx)?
    }

    pub fn request(
        &self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Uuid,
    ) -> Result<Option<RequestedFile>, Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::Request { file_uuid, from_station_uuid, resp: tx }, rx)?
    }

    pub fn stations(&self) -> Result<Vec<StationInfo>, Error> {
        let (tx, rx) = flume::bounded(1);
        self.call(ActorMessage::GetStations { resp: tx }, rx)
    }

    pub fn shutdown(&self) {
        let (tx, rx) = flume::bounded(1);
        let _ = self.call(ActorMessage::Shutdown { resp: tx }, rx);
    }
}

struct StationActor {
    settings: Settings,
    rx: mpsc::Receiver<ActorMessage>,
    events: broadcast::Sender<StationEvent>,
    stations: HashMap<Uuid, StationInfo>,
    local_mvr_files: HashMap<Uuid, LocalMvrFile>,
    latest_file: Option<Uuid>,
}

impl StationActor {
    fn new(
        settings: Settings,
        rx: mpsc::Receiver<ActorMessage>,
        events: broadcast::Sender<StationEvent>,
    ) -> Self {
        Self {
            settings,
            rx,
            events,
            stations: HashMap::new(),
            local_mvr_files: HashMap::new(),
            latest_file: None,
        }
    }

    async fn run(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind((self.settings.interface_ip, self.settings.port)).await?;
        let mdns = mdns_sd::ServiceDaemon::new()?;
        let browser = mdns.browse(SERVICE_TYPE)?;

        let mut reg_tick = time::interval(REGISTRATION_INTERVAL);
        let mut purge_tick = time::interval(PURGE_INTERVAL);

        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    self.spawn_connection_handler(stream, addr);
                }

                Ok(mdns_event) = async { browser.recv_async().await } => {
                    if let ServiceEvent::ServiceResolved(info) = mdns_event {
                        self.handle_mdns_resolved(*info).await;
                    }
                }

                Some(msg) = self.rx.recv() => {
                    match msg {
                        ActorMessage::Shutdown { resp } => {
                            let _ = resp.send(());
                            return Ok(());
                        }
                        m => self.handle_command(m).await,
                    }
                }

                _ = reg_tick.tick() => self.register_mdns(&mdns),
                _ = purge_tick.tick() => self.purge_stale(),
            }
        }
    }

    async fn handle_command(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::InsertLocalFile { file, bytes, resp } => {
                let uuid = file.file_hash_uuid();
                self.local_mvr_files.insert(uuid, LocalMvrFile { mvr_file: Arc::new(file), bytes });
                self.latest_file = Some(uuid);
                let _ = resp.send(Ok(uuid));
            }
            ActorMessage::GetLocalFiles { resp } => {
                let files =
                    self.local_mvr_files.values().map(|f| Arc::clone(&f.mvr_file)).collect();
                let _ = resp.send(files);
            }
            ActorMessage::GetLocalFile { file_uuid, resp } => {
                let file = self.local_mvr_files.get(&file_uuid).map(|f| Arc::clone(&f.mvr_file));
                let _ = resp.send(file);
            }
            ActorMessage::UnloadLocalFile { file_uuid, resp } => {
                if self.local_mvr_files.remove(&file_uuid).is_some() {
                    if self.latest_file == Some(file_uuid) {
                        self.latest_file = self.local_mvr_files.keys().next().copied();
                    }
                    let _ = resp.send(Ok(()));
                } else {
                    let _ = resp.send(Err(Error::LocalMvrFileNotFound { uuid: file_uuid }));
                }
            }
            ActorMessage::Join { target_uuid, resp } => {
                let res = self.do_join(target_uuid).await;
                let _ = resp.send(res);
            }
            ActorMessage::Commit { target_uuid, file_uuid, comment, resp } => {
                let res = self.do_commit(target_uuid, file_uuid, comment).await;
                let _ = resp.send(res);
            }
            ActorMessage::Request { file_uuid, from_station_uuid, resp } => {
                let res = self.do_request(file_uuid, from_station_uuid).await;
                let _ = resp.send(res);
            }
            ActorMessage::GetStations { resp } => {
                let _ = resp.send(self.stations.values().cloned().collect());
            }
            ActorMessage::Shutdown { .. } => {}
        }
    }

    fn spawn_connection_handler(&self, stream: TcpStream, _addr: SocketAddr) {
        let events = self.events.clone();

        tokio::spawn(async move {
            let mut framed = Framed::new(stream, PacketCodec);

            while let Some(msg) = framed.next().await {
                let Ok(packet) = msg else { break };

                match packet.payload {
                    PacketPayload::Commit { file_uuid, station_uuid, .. } => {
                        let info = StationInfo::new(
                            station_uuid,
                            String::new(),
                            SocketAddr::from(([127, 0, 0, 1], 0)),
                            String::new(),
                            SUPPORTED_MVR_FILE_VERSION_MAJOR,
                            SUPPORTED_MVR_FILE_VERSION_MINOR,
                            Vec::new(),
                        );
                        let _ = events.send(StationEvent::Committed { station: info, file_uuid });
                    }
                    PacketPayload::Request { file_uuid, .. } => {
                        let info = StationInfo::new(
                            Uuid::nil(),
                            String::new(),
                            SocketAddr::from(([127, 0, 0, 1], 0)),
                            String::new(),
                            SUPPORTED_MVR_FILE_VERSION_MAJOR,
                            SUPPORTED_MVR_FILE_VERSION_MINOR,
                            Vec::new(),
                        );
                        let _ = events.send(StationEvent::Requested { station: info, file_uuid });
                    }
                    _ => {}
                }
            }
        });
    }

    fn register_mdns(&self, mdns: &mdns_sd::ServiceDaemon) {
        let host = format!("{}.local.", self.settings.station_name);
        let props = HashMap::from([
            ("StationUUID".to_string(), self.settings.station_uuid.to_string()),
            ("Provider".to_string(), self.settings.provider_name.clone()),
            ("VersionMajor".to_string(), SUPPORTED_MVR_FILE_VERSION_MAJOR.to_string()),
            ("VersionMinor".to_string(), SUPPORTED_MVR_FILE_VERSION_MINOR.to_string()),
        ]);

        if let Ok(info) = ServiceInfo::new(
            SERVICE_TYPE,
            &self.settings.group_name,
            &host,
            self.settings.interface_ip,
            self.settings.port,
            props,
        ) {
            let _ = mdns.register(info);
        }
    }

    async fn handle_mdns_resolved(&mut self, info: ResolvedService) {
        let Some(uuid_str) = info.get_property_val_str("StationUUID") else {
            return;
        };

        let Ok(uuid) = Uuid::parse_str(uuid_str) else {
            return;
        };

        if uuid == self.settings.station_uuid {
            return;
        }

        let Some(addr) = info.get_addresses().iter().next() else {
            return;
        };

        let station = StationInfo::new(
            uuid,
            info.get_fullname().to_string(),
            SocketAddr::new(addr.to_ip_addr(), info.get_port()),
            info.get_property_val_str("Provider")
                .map(ToString::to_string)
                .unwrap_or_else(|| "Unknown".to_string()),
            SUPPORTED_MVR_FILE_VERSION_MAJOR,
            SUPPORTED_MVR_FILE_VERSION_MINOR,
            Vec::new(),
        );

        let is_new = !self.stations.contains_key(&uuid);
        self.stations.insert(uuid, station.clone());

        if is_new {
            let _ = self.events.send(StationEvent::Joined(station));
        }
    }

    fn purge_stale(&mut self) {
        let now = Instant::now();
        let mut left = Vec::new();

        self.stations.retain(|_, s| {
            let keep = now.duration_since(s.last_seen) < STALE_THRESHOLD;
            if !keep {
                left.push(s.clone());
            }
            keep
        });

        for station in left {
            let _ = self.events.send(StationEvent::Left(station));
        }
    }

    async fn do_join(&mut self, target_uuid: Uuid) -> Result<(), Error> {
        let addr = self
            .stations
            .get(&target_uuid)
            .map(|s| s.address)
            .ok_or(Error::StationNotFound { uuid: target_uuid })?;

        let stream = TcpStream::connect(addr).await?;
        self.spawn_connection_handler(stream, addr);
        Ok(())
    }

    async fn do_commit(
        &mut self,
        target_uuid: Uuid,
        file_uuid: Uuid,
        comment: Option<String>,
    ) -> Result<(), Error> {
        let station = self
            .stations
            .get(&target_uuid)
            .cloned()
            .ok_or(Error::StationNotFound { uuid: target_uuid })?;

        let Some(local) = self.local_mvr_files.get(&file_uuid) else {
            return Err(Error::LocalMvrFileNotFound { uuid: file_uuid });
        };

        let stream = TcpStream::connect(station.address).await?;
        let mut framed = Framed::new(stream, PacketCodec);

        let mvr = &local.mvr_file;
        let payload = PacketPayload::Commit {
            file_uuid,
            station_uuid: self.settings.station_uuid,
            file_size: local.bytes.len() as u64,
            ver_major: mvr.general_scene_description().ver_major,
            ver_minor: mvr.general_scene_description().ver_minor,
            for_stations_uuid: vec![target_uuid],
            file_name: local
                .mvr_file
                .file_path()
                .and_then(|path| path.file_name().map(|s| s.to_string_lossy().to_string())),
            comment,
        };

        let packet = Packet::new(payload, 0, 1)?;
        framed.send(packet).await.map_err(Error::from)
    }

    async fn do_request(
        &mut self,
        file_uuid: Option<Uuid>,
        from_station_uuid: Uuid,
    ) -> Result<Option<RequestedFile>, Error> {
        let station = self
            .stations
            .get(&from_station_uuid)
            .cloned()
            .ok_or(Error::StationNotFound { uuid: from_station_uuid })?;

        let _stream = TcpStream::connect(station.address).await?;

        let chosen_uuid = match file_uuid {
            Some(u) => u,
            None => match self.latest_file {
                Some(u) => u,
                None => return Ok(None),
            },
        };

        let Some(local) = self.local_mvr_files.get(&chosen_uuid) else {
            return Ok(None);
        };

        Ok(Some(RequestedFile {
            uuid: chosen_uuid,
            name: local
                .mvr_file
                .file_path()
                .and_then(|path| path.file_name().map(|s| s.to_string_lossy().to_string())),
            bytes: local.bytes.clone(),
        }))
    }
}

struct LocalMvrFile {
    mvr_file: Arc<MvrFile>,
    bytes: Bytes,
}

/// A file received in response to an `MVR_REQUEST`.
pub struct RequestedFile {
    uuid: Uuid,
    name: Option<String>,
    bytes: Bytes,
}

impl RequestedFile {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }
}
