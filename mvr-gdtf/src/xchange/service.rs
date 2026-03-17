use std::{
    collections::{HashMap, HashSet},
    io,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use mdns_sd::{ServiceEvent, ServiceInfo};
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
            port: 48484,
        }
    }
}

pub struct Service {
    settings: Settings,
    inner: Arc<Inner>,
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

        Ok(Self { settings, inner })
    }

    pub fn start(&self) -> Result<(), crate::xchange::Error> {
        self.start_listener()?;
        self.start_mdns_registration()?;
        self.start_mdns_browser_and_join()?;

        Ok(())
    }

    fn start_listener(&self) -> Result<(), io::Error> {
        let listener = TcpListener::bind((self.settings.interface_ip, self.settings.port))?;
        let inner = Arc::clone(&self.inner);

        thread::spawn(move || {
            for stream_result in listener.incoming() {
                let Ok(stream) = stream_result else {
                    log::error!("failed to accept connection: {}", stream_result.unwrap_err());
                    continue;
                };

                let inner = Arc::clone(&inner);
                thread::spawn(move || {
                    if let Err(err) = inner.handle_connection(stream) {
                        log::error!("failed to handle connection: {}", err);
                    }
                });
            }
        });

        Ok(())
    }

    fn start_mdns_registration(&self) -> Result<(), crate::xchange::Error> {
        let mdns = mdns_sd::ServiceDaemon::new()?;
        let inner = Arc::clone(&self.inner);

        thread::spawn(move || {
            loop {
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
                    thread::sleep(REGISTRATION_INTERVAL);
                    continue;
                };

                if let Err(err) = mdns.register(service) {
                    log::error!("failed to register mDNS service: {err}");
                }

                thread::sleep(REGISTRATION_INTERVAL);
            }
        });

        Ok(())
    }

    fn start_mdns_browser_and_join(&self) -> Result<(), crate::xchange::Error> {
        let mdns = mdns_sd::ServiceDaemon::new()?;
        let browser = mdns.browse(SERVICE_TYPE)?;
        let inner = Arc::clone(&self.inner);
        let settings = self.settings.clone();

        thread::spawn(move || {
            while let Ok(event) = browser.recv() {
                let ServiceEvent::ServiceResolved(service) = event else { continue };

                let Some(uuid_str) = service.get_property_val_str("StationUUID") else {
                    continue;
                };
                let Ok(uuid) = Uuid::parse_str(uuid_str) else { continue };

                // Prevent the service from attempting to handshake with itself.
                if uuid == settings.station_uuid {
                    continue;
                }

                for addr in service.get_addresses() {
                    let socket_addr = SocketAddr::from((addr.to_ip_addr(), service.get_port()));

                    if inner.station_is_registered(&uuid) {
                        continue;
                    }

                    if let Err(err) = inner.send_join(socket_addr) {
                        log::warn!("failed to send MVR_JOIN to {socket_addr}: {err}");
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stations_in_group(&self) -> Vec<StationInfo> {
        let joined_uuids = self.inner.joined_stations.lock().unwrap();
        let stations = self.inner.stations.lock().unwrap();
        joined_uuids.iter().filter_map(|uuid| stations.get(uuid).cloned()).collect()
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
                self.join_station(StationInfo {
                    uuid: station_uuid,
                    name: station_name,
                    address: stream.peer_addr()?,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                });
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
                self.join_station(StationInfo {
                    uuid: station_uuid,
                    name: station_name,
                    address: stream.peer_addr()?,
                    provider,
                    ver_major,
                    ver_minor,
                    commits,
                });

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

    fn station_is_registered(&self, station_uuid: &Uuid) -> bool {
        self.stations.lock().unwrap().contains_key(station_uuid)
    }

    fn join_station(&self, info: StationInfo) {
        self.joined_stations.lock().unwrap().insert(info.uuid);
        self.stations.lock().unwrap().insert(info.uuid, info);
    }

    fn leave_station(&self, uuid: &Uuid) {
        self.joined_stations.lock().unwrap().remove(uuid);
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
}

impl StationInfo {
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
