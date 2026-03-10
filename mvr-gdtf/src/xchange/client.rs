use std::{
    io::{Read as _, Write},
    net::TcpStream,
    thread::{self},
};

use mdns_sd::{ServiceEvent, ServiceInfo};
use uuid::Uuid;

use crate::xchange::packet::{Packet, PacketHeader, PacketPayload};

const SERVICE_TYPE: &str = "_mvrxchange._tcp.local.";

pub struct Client {
    group_name: String,
    station_name: String,
    station_uuid: Uuid,
}

impl Client {
    pub fn new(
        group_name: String,
        station_name: String,
        station_uuid: Uuid,
    ) -> Result<Self, crate::xchange::Error> {
        Ok(Self { group_name, station_name, station_uuid })
    }

    pub fn start(&mut self) {
        let inner = Inner {
            group_name: self.group_name.clone(),
            station_name: self.station_name.clone(),
            station_uuid: self.station_uuid.clone(),
        };

        thread::spawn(move || {
            if let Err(err) = inner.run() {
                eprintln!("Client thread error: {:?}", err);
            }
        });
    }

    // Add other send functions here.
    pub fn send_join() -> () {}
}

struct Inner {
    group_name: String,
    station_name: String,
    station_uuid: Uuid,
}

impl Inner {
    fn run(self) -> Result<(), crate::xchange::Error> {
        // Start a mDNS service.

        let mdns = mdns_sd::ServiceDaemon::new().unwrap();

        let host_name = format!("{}.{}", self.group_name, SERVICE_TYPE);
        let properties = [
            ("StationName", self.station_name.clone()),
            ("StationUUID", self.station_uuid.to_string()),
        ];

        let service = mdns_sd::ServiceInfo::new(
            SERVICE_TYPE,
            &self.group_name,
            &host_name,
            local_ip_address::local_ip().unwrap(),
            4457,
            &properties[..],
        )
        .unwrap();

        mdns.register(service).unwrap();

        // Browse for all with group_name (except ourselves).

        let browser = mdns.browse(SERVICE_TYPE).unwrap();
        loop {
            match browser.recv() {
                Ok(event) => match event {
                    ServiceEvent::ServiceResolved(service) => {
                        self.handle_service(service)?;
                    }
                    _ => {}
                },
                Err(err) => todo!("{err}"),
            }
        }
    }

    fn handle_service(&self, service: ServiceInfo) -> Result<(), crate::xchange::Error> {
        let fullname = service.get_fullname();
        let service_group_name = fullname
            .strip_suffix(SERVICE_TYPE)
            .unwrap_or(fullname)
            .trim_end_matches('.')
            .to_string();

        if self.group_name != service_group_name {
            return Ok(());
        }

        let port = service.get_port();

        let mut properties = std::collections::HashMap::new();
        for prop in service.get_properties().iter() {
            properties.insert(prop.key().to_string(), prop.val_str().to_string());
        }

        for ip in service.get_addresses().iter().copied() {
            let station_addr = std::net::SocketAddr::new(std::net::IpAddr::from(ip), port);

            let station_uuid = match properties.get("StationUUID") {
                Some(uuid_str) => match Uuid::parse_str(uuid_str) {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        eprintln!("Discovered station with invalid UUID: {}", uuid_str);
                        continue;
                    }
                },
                None => {
                    eprintln!("Discovered station missing UUID");
                    continue;
                }
            };

            // Skip ourselves
            if station_uuid == self.station_uuid {
                continue;
            }

            let mut socket = TcpStream::connect(station_addr)?;
            socket.set_nodelay(true)?;

            self.send_join(&mut socket)?;

            loop {
                let packet = Packet::read(&mut socket).unwrap();
                dbg!(packet);
            }
        }

        Ok(())
    }

    pub fn send_join(&self, socket: &mut TcpStream) -> Result<(), crate::xchange::Error> {
        let packet = Packet::from_payload(
            PacketPayload::MvrJoin {
                provider: "Provider Name".to_string(),
                station_name: self.station_name.clone(),
                ver_major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                ver_minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                station_uuid: self.station_uuid,
                commits: Vec::new(),
            },
            0,
            1,
        );

        packet.write(socket)?;

        Ok(())
    }

    pub fn send_leave(&self, socket: &mut TcpStream) -> Result<(), crate::xchange::Error> {
        let packet = Packet::from_payload(
            PacketPayload::MvrLeave { from_station_uuid: self.station_uuid },
            0,
            1,
        );
        packet.write(socket)?;
        Ok(())
    }

    pub fn send_commit(
        &self,
        socket: &mut TcpStream,
        args: crate::xchange::packet::MvrCommitArgs,
    ) -> Result<(), crate::xchange::Error> {
        let packet = Packet::from_payload(
            PacketPayload::MvrCommit {
                file_uuid: args.file_uuid,
                station_uuid: args.station_uuid,
                file_size: args.file_size,
                ver_major: args.ver_major,
                ver_minor: args.ver_minor,
                for_stations_uuid: args.for_stations_uuid,
                file_name: args.file_name,
                comment: args.comment,
            },
            0,
            1,
        );
        packet.write(socket)?;
        Ok(())
    }

    pub fn send_request(
        &self,
        socket: &mut TcpStream,
        file_uuid: Option<Uuid>,
        from_station_uuid: Vec<Uuid>,
    ) -> Result<(), crate::xchange::Error> {
        let packet =
            Packet::from_payload(PacketPayload::MvrRequest { file_uuid, from_station_uuid }, 0, 1);
        packet.write(socket)?;
        Ok(())
    }

    pub fn send_new_session_host(
        &self,
        socket: &mut TcpStream,
        service_name: Option<String>,
        service_url: Option<String>,
    ) -> Result<(), crate::xchange::Error> {
        let packet = Packet::from_payload(
            PacketPayload::MvrNewSessionHost { service_name, service_url },
            0,
            1,
        );
        packet.write(socket)?;
        Ok(())
    }

    pub fn send_file(
        &self,
        socket: &mut TcpStream,
        data: Vec<u8>,
        package_number: u32,
        package_count: u32,
    ) -> Result<(), crate::xchange::Error> {
        let packet = Packet::mvr_file(data, package_number, package_count);
        packet.write(socket)?;
        Ok(())
    }
}
