use std::{
    collections::HashMap,
    io,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use mdns_sd::{ResolvedService, ServiceEvent, ServiceInfo};
use uuid::Uuid;

use crate::xchange::packet::{Commit, Packet, PacketPayload};

pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";
const REGISTRATION_INTERVAL: Duration = Duration::from_secs(10);

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
            port: 5568,
        }
    }
}

pub struct Service {
    inner: Arc<Inner>,
}

impl Service {
    pub fn new(mut settings: Settings) -> Result<Self, crate::xchange::Error> {
        // mDNS names must not contain spaces, slashes, or certain punctuation.
        let lossy_station_name: String = settings
            .station_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '-' })
            .collect();
        settings.station_name = lossy_station_name;

        let mdns = mdns_sd::ServiceDaemon::new()?;

        thread::spawn({
            let mdns = mdns.clone();
            let settings = settings.clone();
            move || {
                loop {
                    let service = match ServiceInfo::new(
                        SERVICE_TYPE,
                        &settings.group_name,
                        &format!("{}.local.", settings.station_name),
                        &settings.interface_ip.to_string(),
                        settings.port,
                        HashMap::from([
                            ("StationName".to_string(), settings.station_name.to_string()),
                            ("StationUUID".to_string(), settings.station_uuid.to_string()),
                        ]),
                    ) {
                        Ok(service) => service,
                        Err(err) => {
                            log::error!("Failed to create mDNS service: {err}");
                            continue;
                        }
                    };

                    if let Err(err) = mdns.register(service) {
                        log::error!("Failed to register mDNS service: {err}");
                    }

                    thread::sleep(REGISTRATION_INTERVAL);
                }
            }
        });

        let browser = mdns.browse(SERVICE_TYPE)?;

        let inner = Arc::new(Inner::new(settings)?);

        thread::spawn({
            let inner = Arc::clone(&inner);
            move || {
                loop {
                    match browser.recv() {
                        Ok(ServiceEvent::ServiceResolved(service)) => {
                            if let Err(e) = inner.update_stations_for_service(&service) {
                                log::error!("Failed to update stations: {:?}", e);
                            }
                        }
                        Err(_) => break,
                        _ => {}
                    }
                }
            }
        });

        Ok(Self { inner })
    }

    pub fn stations(&self) -> Vec<StationInfo> {
        self.inner
            .connections
            .read()
            .unwrap()
            .values()
            .filter_map(|conn| conn.station_info.clone())
            .collect()
    }
}

struct Inner {
    settings: Settings,

    connections: RwLock<HashMap<SocketAddr, Connection>>,
}

impl Inner {
    pub fn new(settings: Settings) -> Result<Self, io::Error> {
        let listener = TcpListener::bind((settings.interface_ip, settings.port))?;

        thread::spawn(move || {
            for connection in listener.incoming() {
                let stream = connection.unwrap();
                while let Ok(packet) = Packet::read(&stream) {
                    println!("listener_packet: {packet:?}");
                }
            }
        });

        Ok(Self { settings, connections: RwLock::new(HashMap::new()) })
    }

    pub fn update_stations_for_service(
        &self,
        service: &ResolvedService,
    ) -> Result<(), crate::xchange::Error> {
        let port = service.get_port();
        let addresses = service
            .get_addresses()
            .iter()
            .map(|ip| SocketAddr::from((ip.to_ip_addr(), port)))
            .collect::<Vec<_>>();
        for addr in &addresses {
            // Do not add ourselves.
            if (addr.ip(), port) == (self.settings.interface_ip, self.settings.port) {
                continue;
            }

            let mut connection = self.take_or_create_connection(*addr)?;
            if connection.join(&self.settings).is_ok() {
                self.connections.write().unwrap().insert(*addr, connection);
            }
        }

        self.connections.write().unwrap().retain(|addr, _| addresses.contains(&addr));

        Ok(())
    }

    fn take_or_create_connection(&self, socket_addr: SocketAddr) -> Result<Connection, io::Error> {
        match self.connections.write().unwrap().remove(&socket_addr) {
            Some(conn) => Ok(conn),
            None => Ok(Connection::new(TcpStream::connect(socket_addr)?)?),
        }
    }
}

struct Connection {
    stream: TcpStream,
    station_info: Option<StationInfo>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Result<Self, io::Error> {
        thread::spawn({
            let stream = stream.try_clone()?;
            move || {
                while let Ok(packet) = Packet::read(&stream) {
                    println!("conn_packet: {packet:?}");
                }
            }
        });

        Ok(Self { stream, station_info: None })
    }

    pub fn update_station_info(&mut self, station_info: StationInfo) {
        self.station_info = Some(station_info);
    }
}

impl Connection {
    pub fn join(&mut self, settings: &Settings) -> Result<(), crate::xchange::Error> {
        let join_packet = Packet::new(
            PacketPayload::Join {
                provider: settings.provider_name.to_owned(),
                station_name: settings.station_name.to_owned(),
                station_uuid: settings.station_uuid,
                ver_major: settings.ver_major,
                ver_minor: settings.ver_minor,
                commits: Vec::new(),
            },
            0,
            1,
        )?;

        join_packet.write(&self.stream)?;

        match Packet::read(&self.stream) {
            Ok(Packet { payload, .. }) => match payload {
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
                    log::debug!(
                        "Station '{station_name}' ({station_uuid}) joined successfully: provider={provider}, version={ver_major}.{ver_minor}",
                    );

                    self.update_station_info(StationInfo {
                        uuid: station_uuid,
                        name: station_name,
                        addr: self.stream.peer_addr()?,
                        provider,
                        ver_major,
                        ver_minor,
                        commits,
                    });
                }
                PacketPayload::JoinRet {
                    ok: false, message, station_name, station_uuid, ..
                } => {
                    log::warn!("Station '{station_name} ({station_uuid}) rejected join: {message}");
                }
                payload => {
                    log::warn!("Unexpected payload: {:?}", payload);
                }
            },
            Err(err) => return Err(err),
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationInfo {
    uuid: Uuid,
    name: String,
    addr: SocketAddr,
    provider: String,
    ver_major: u32,
    ver_minor: u32,
    commits: Vec<Commit>,
}

impl StationInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
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
