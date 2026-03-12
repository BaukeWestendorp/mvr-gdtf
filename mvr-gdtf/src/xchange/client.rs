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

const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";

pub enum Command {
    RegisterStation {
        name: String,
        uuid: Uuid,
        provider: String,
        ver_major: u32,
        ver_minor: u32,
        commits: Vec<Commit>,
        stream: TcpStream,
    },
    RemoveStation {
        uuid: Uuid,
    },
}

pub struct Client {
    group_name: String,
    station_name: String,
    station_uuid: Uuid,

    station_infos: Arc<Mutex<HashMap<Uuid, StationInfo>>>,
    station_streams: Arc<Mutex<HashMap<Uuid, TcpStream>>>,
}

impl Client {
    pub fn new(
        group_name: String,
        station_name: String,
        station_uuid: Uuid,
    ) -> Result<Self, crate::xchange::Error> {
        let (cmd_tx, cmd_rx) = flume::unbounded();

        let client = Self {
            group_name,
            station_name,
            station_uuid,
            station_infos: Arc::new(Mutex::new(HashMap::new())),
            station_streams: Arc::new(Mutex::new(HashMap::new())),
        };

        client.start_manager(cmd_rx, cmd_tx.clone());
        client.start_discovery(cmd_tx);

        Ok(client)
    }

    pub fn stations(&self) -> HashMap<Uuid, StationInfo> {
        self.station_infos.lock().unwrap().clone()
    }

    fn start_manager(&self, cmd_rx: Receiver<Command>, cmd_tx: Sender<Command>) {
        thread::spawn({
            let station_infos = Arc::clone(&self.station_infos);
            let station_streams = Arc::clone(&self.station_streams);
            move || {
                while let Ok(cmd) = cmd_rx.recv() {
                    match cmd {
                        Command::RegisterStation {
                            name,
                            uuid,
                            stream,
                            provider,
                            ver_major,
                            ver_minor,
                            commits,
                        } => {
                            log::trace!("Registering station: uuid = {}", uuid);

                            station_infos.lock().unwrap().insert(
                                uuid,
                                StationInfo {
                                    name,
                                    addr: stream.peer_addr().unwrap(),
                                    provider,
                                    ver_major,
                                    ver_minor,
                                    commits,
                                },
                            );

                            thread::spawn({
                                let cmd_tx = cmd_tx.clone();
                                let stream = stream.try_clone().expect("Failed to clone stream");
                                move || {
                                    loop {
                                        match Packet::read(&stream) {
                                            Ok(packet) => log::debug!("{:?}", packet),
                                            Err(_) => break,
                                        }
                                    }
                                    let _ = cmd_tx.send(Command::RemoveStation { uuid });
                                }
                            });

                            station_streams.lock().unwrap().insert(uuid, stream);
                        }
                        Command::RemoveStation { uuid } => {
                            log::trace!("Removing station: uuid = {}", uuid);

                            station_infos.lock().unwrap().remove(&uuid);
                            if let Some(stream) = station_streams.lock().unwrap().remove(&uuid) {
                                let _ = stream.shutdown(Shutdown::Both);
                            }
                        }
                    }
                }
            }
        });
    }

    fn start_discovery(&self, cmd_tx: Sender<Command>) {
        let group_name = self.group_name.clone();
        let station_name = self.station_name.clone();
        let station_uuid = self.station_uuid;

        thread::spawn(move || {
            let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");

            thread::spawn({
                let mdns = mdns.clone();
                let group_name = group_name.clone();
                let station_name = station_name.clone();
                move || {
                    let host_name = format!("{}.{}", group_name, SERVICE_TYPE);
                    let properties = HashMap::from([
                        ("StationName".to_string(), station_name),
                        ("StationUUID".to_string(), station_uuid.to_string()),
                    ]);

                    loop {
                        let new_service = ServiceInfo::new(
                            SERVICE_TYPE,
                            &group_name,
                            &host_name,
                            local_ip_address::local_ip().unwrap().to_string(),
                            5353,
                            properties.clone(),
                        )
                        .expect("Failed to create service info");

                        mdns.register(new_service).ok();
                        thread::sleep(Duration::from_secs(10));
                    }
                }
            });

            let browser = mdns.browse(SERVICE_TYPE).expect("Failed to browse mDNS");
            while let Ok(event) = browser.recv() {
                if let ServiceEvent::ServiceResolved(info) = event {
                    handle_discovered_service(&group_name, station_uuid, info, &cmd_tx);
                }
            }
        });
    }
}

fn handle_discovered_service(
    group_name: &str,
    local_uuid: Uuid,
    service: ServiceInfo,
    cmd_tx: &Sender<Command>,
) {
    let fullname = service.get_fullname();
    let service_group_name =
        fullname.strip_suffix(SERVICE_TYPE).unwrap_or(fullname).trim_end_matches('.').to_string();

    if group_name != service_group_name {
        return;
    }

    let properties: HashMap<String, String> = service
        .get_properties()
        .iter()
        .map(|p| (p.key().to_string(), p.val_str().to_string()))
        .collect();

    let remote_uuid = match properties.get("StationUUID").and_then(|s| Uuid::parse_str(s).ok()) {
        Some(uuid) => uuid,
        None => return,
    };

    if remote_uuid == local_uuid {
        return;
    }

    let port = service.get_port();
    for ip in service.get_addresses() {
        let remote_addr = SocketAddr::new(IpAddr::from(*ip), port);
        if let Ok(stream) = TcpStream::connect_timeout(&remote_addr, Duration::from_millis(500)) {
            let join_packet = Packet::new(
                PacketPayload::Join {
                    provider: "mvr-gdtf".to_string(),
                    station_name: "Rust Station Name".to_string(),
                    station_uuid: Uuid::new_v4(),
                    ver_major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                    ver_minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                    commits: Vec::new(),
                },
                0,
                1,
            )
            .unwrap();

            if let Err(e) = join_packet.write(&stream) {
                log::error!("Failed to write join packet: {}", e);
                continue;
            }

            let ret_packet = match Packet::read(&stream) {
                Ok(packet) => packet,
                Err(e) => {
                    log::error!("Failed to read return packet: {}", e);
                    continue;
                }
            };

            match ret_packet.payload {
                PacketPayload::JoinRet {
                    ok,
                    message,
                    provider,
                    station_name,
                    station_uuid,
                    ver_major,
                    ver_minor,
                    commits,
                } => {
                    if ok {
                        let _ = cmd_tx.send(Command::RegisterStation {
                            name: station_name,
                            uuid: station_uuid,
                            provider,
                            ver_major,
                            ver_minor,
                            commits,
                            stream,
                        });
                    } else {
                        log::warn!(
                            "Station {} ({}) rejected join: {}",
                            station_name,
                            station_uuid,
                            message
                        );

                        continue;
                    }
                }
                _ => {
                    log::warn!("Received unexpected packet payload: {:?}", ret_packet.payload);
                    continue;
                }
            }
        }
    }
}

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
