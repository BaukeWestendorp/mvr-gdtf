use std::{
    collections::HashMap,
    net::{IpAddr, Shutdown, SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use flume::{Receiver, Sender};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use uuid::Uuid;

use crate::xchange::packet::{Commit, Packet, PacketPayload};

pub const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";
const CONNECT_TIMEOUT: Duration = Duration::from_millis(500);
const REREGISTER_INTERVAL: Duration = Duration::from_secs(10);

enum ManagerCommand {
    Register {
        name: String,
        uuid: Uuid,
        provider: String,
        ver_major: u32,
        ver_minor: u32,
        commits: Vec<Commit>,
        stream: TcpStream,
    },
    Remove {
        uuid: Uuid,
    },
}

struct PacketWriter<'a>(&'a TcpStream);
struct PacketReader<'a>(&'a TcpStream);

impl<'a> PacketWriter<'a> {
    fn send(
        &self,
        payload: PacketPayload,
        msg_id: u32,
        ver: u32,
    ) -> Result<(), crate::xchange::Error> {
        Packet::new(payload, msg_id, ver)?.write(self.0)
    }
}

impl<'a> PacketReader<'a> {
    fn recv(&self) -> Result<Packet, crate::xchange::Error> {
        Packet::read(self.0)
    }
}

fn writer(stream: &TcpStream) -> PacketWriter<'_> {
    PacketWriter(stream)
}

fn reader(stream: &TcpStream) -> PacketReader<'_> {
    PacketReader(stream)
}

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

pub struct Client {
    settings: Settings,

    station_infos: Arc<Mutex<HashMap<Uuid, StationInfo>>>,
    station_streams: Arc<Mutex<HashMap<Uuid, TcpStream>>>,
}

impl Client {
    pub fn new(settings: Settings) -> Result<Self, crate::xchange::Error> {
        let (cmd_tx, cmd_rx) = flume::unbounded();

        let client = Self {
            settings,

            station_infos: Arc::new(Mutex::new(HashMap::new())),
            station_streams: Arc::new(Mutex::new(HashMap::new())),
        };

        log::debug!("Starting MVR-xchange client");
        // client.start_discovery(cmd_tx.clone());
        // client.start_manager(cmd_rx, cmd_tx);

        Ok(client)
    }

    pub fn stations(&self) -> HashMap<Uuid, StationInfo> {
        self.station_infos.lock().unwrap().clone()
    }

    // fn start_manager(&self, cmd_rx: Receiver<ManagerCommand>, cmd_tx: Sender<ManagerCommand>) {
    //     let station_infos = Arc::clone(&self.station_infos);
    //     let station_streams = Arc::clone(&self.station_streams);

    //     thread::spawn(move || {
    //         log::trace!("Started manager thread");

    //         while let Ok(cmd) = cmd_rx.recv() {
    //             match cmd {
    //                 ManagerCommand::Register {
    //                     name,
    //                     uuid,
    //                     stream,
    //                     provider,
    //                     ver_major,
    //                     ver_minor,
    //                     commits,
    //                 } => {
    //                     log::trace!("Registering station: {uuid}");

    //                     station_infos.lock().unwrap().insert(
    //                         uuid,
    //                         StationInfo {
    //                             name,
    //                             addr: stream.peer_addr().unwrap(),
    //                             provider,
    //                             ver_major,
    //                             ver_minor,
    //                             commits,
    //                         },
    //                     );

    //                     spawn_station_reader(
    //                         uuid,
    //                         stream.try_clone().expect("clone stream"),
    //                         cmd_tx.clone(),
    //                     );
    //                     station_streams.lock().unwrap().insert(uuid, stream);
    //                 }

    //                 ManagerCommand::Remove { uuid } => {
    //                     log::trace!("Removing station: {uuid}");

    //                     station_infos.lock().unwrap().remove(&uuid);
    //                     if let Some(stream) = station_streams.lock().unwrap().remove(&uuid) {
    //                         let _ = stream.shutdown(Shutdown::Both);
    //                     }
    //                 }
    //             }
    //         }
    //     });
    // }

    // fn start_discovery(&self, cmd_tx: Sender<ManagerCommand>) {
    //     let settings = self.settings.clone();
    //     let station_infos = Arc::clone(&self.station_infos);

    //     thread::spawn(move || {
    //         log::trace!("Started discovery thread");

    //         let mdns = ServiceDaemon::new().expect("mDNS daemon");

    //         spawn_registrator(mdns.clone(), settings.clone());

    //         let browser = mdns.browse(SERVICE_TYPE).expect("mDNS browse");
    //         loop {
    //             match browser.recv() {
    //                 Ok(ServiceEvent::ServiceResolved(info)) => {
    //                     handle_discovered_service(info, &settings, &cmd_tx, &station_infos);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     });
    // }
}

// fn spawn_station_reader(station_uuid: Uuid, stream: TcpStream, cmd_tx: Sender<ManagerCommand>) {
//     thread::spawn(move || {
//         log::trace!("Started station reader thread");

//         let rx = reader(&stream);
//         loop {
//             match rx.recv() {
//                 Ok(packet) => handle_inbound_packet(packet),
//                 Err(_) => break,
//             }
//         }
//         let _ = cmd_tx.send(ManagerCommand::Remove { uuid: station_uuid });
//     });
// }

// fn handle_inbound_packet(packet: Packet) {
//     match &packet.payload {
//         _ => {
//             log::debug!("Received packet: {:?}", packet);
//         }
//     }
// }

// fn spawn_registrator(mdns: ServiceDaemon, settings: Settings) {
//     thread::spawn(move || {
//         log::trace!("Started registrator thread");

//         let host_name = format!("{}.local.", settings.station_name);
//         let properties = HashMap::from([
//             ("StationName".to_string(), settings.station_name),
//             ("StationUUID".to_string(), settings.station_uuid.to_string()),
//         ]);

//         loop {
//             log::trace!("Registering mDNS service...");

// let service = ServiceInfo::new(
//     SERVICE_TYPE,
//     &settings.group_name,
//     &host_name,
//     settings.interface_ip,
//     settings.port,
//     properties.clone(),
// )
// .expect("ServiceInfo");

//             let _ = mdns.register(service);
//             thread::sleep(REREGISTER_INTERVAL);
//         }
//     });
// }

// fn handle_discovered_service(
//     service: ServiceInfo,
//     settings: &Settings,
//     cmd_tx: &Sender<ManagerCommand>,
//     station_infos: &Arc<Mutex<HashMap<Uuid, StationInfo>>>,
// ) {
//     log::trace!("Discovered a service: {}", service.get_fullname());

//     let service_group = service
//         .get_fullname()
//         .strip_suffix(SERVICE_TYPE)
//         .unwrap_or(service.get_fullname())
//         .trim_end_matches('.');

//     if settings.group_name != service_group {
//         return;
//     }

//     let properties: HashMap<_, _> = service
//         .get_properties()
//         .iter()
//         .map(|p| (p.key().to_string(), p.val_str().to_string()))
//         .collect();

//     // We should not add ourselves
//     let service_uuid = match properties.get("StationUUID").and_then(|s| Uuid::parse_str(s).ok()) {
//         Some(uuid) if uuid != settings.station_uuid => uuid,
//         _ => return,
//     };

//     if station_infos.lock().unwrap().contains_key(&service_uuid) {
//         return;
//     }

//     let port = service.get_port();
//     for ip in service.get_addresses() {
//         let addr = SocketAddr::new(IpAddr::from(*ip), port);
//         if let Err(e) = try_connect(settings, addr, cmd_tx) {
//             log::trace!("Failed to connect to {addr}: {e}");
//         }
//     }
// }

// fn try_connect(
//     settings: &Settings,
//     addr: SocketAddr,
//     cmd_tx: &Sender<ManagerCommand>,
// ) -> Result<(), crate::xchange::Error> {
//     let stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)?;

//     let tx = writer(&stream);
//     let rx = reader(&stream);

//     tx.send(
//         PacketPayload::Join {
//             provider: settings.provider_name.to_owned(),
//             station_name: settings.station_name.to_owned(),
//             station_uuid: settings.station_uuid.to_owned(),
//             ver_major: settings.ver_major,
//             ver_minor: settings.ver_minor,
//             commits: Vec::new(),
//         },
//         0,
//         1,
//     )?;

//     match rx.recv()?.payload {
//         PacketPayload::JoinRet {
//             ok: true,
//             provider,
//             station_name,
//             station_uuid,
//             ver_major,
//             ver_minor,
//             commits,
//             ..
//         } => {
//             if station_uuid == settings.station_uuid {
//                 return Ok(());
//             }

//             cmd_tx
//                 .send(ManagerCommand::Register {
//                     name: station_name,
//                     uuid: station_uuid,
//                     provider,
//                     ver_major,
//                     ver_minor,
//                     commits,
//                     stream,
//                 })
//                 .ok();
//         }
// PacketPayload::JoinRet { ok: false, message, station_name, station_uuid, .. } => {
//     log::warn!("Station {station_name} ({station_uuid}) rejected join: {message}");
// }
//         payload => {
//             log::warn!("Unexpected packet payload: {payload:?}");
//         }
//     }

//     Ok(())
// }

#[derive(Debug, Clone)]
pub struct StationInfo {
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
