use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub package_header: u32,
    pub package_version: u32,
    pub package_number: u32,
    pub package_count: u32,
    pub package_type: u32,
    pub payload_length: u64,
}

impl PacketHeader {
    pub const LEN: usize = 28;

    pub fn new(payload_length: u64, package_number: u32, package_count: u32) -> Self {
        Self {
            package_header: 778682,
            package_version: 1,
            package_number,
            package_count,
            package_type: 0,
            payload_length,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::LEN);
        buf.extend_from_slice(&self.package_header.to_be_bytes());
        buf.extend_from_slice(&self.package_version.to_be_bytes());
        buf.extend_from_slice(&self.package_number.to_be_bytes());
        buf.extend_from_slice(&self.package_count.to_be_bytes());
        buf.extend_from_slice(&self.package_type.to_be_bytes());
        buf.extend_from_slice(&self.payload_length.to_be_bytes());
        buf
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::LEN {
            return None;
        }

        let package_header = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let package_version = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let package_number = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let package_count = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        let package_type = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let payload_length = u64::from_be_bytes([
            bytes[20], bytes[21], bytes[22], bytes[23], bytes[24], bytes[25], bytes[26], bytes[27],
        ]);

        Some(Self {
            package_header,
            package_version,
            package_number,
            package_count,
            package_type,
            payload_length,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvrCommitArgs {
    #[serde(rename = "FileUUID")]
    pub file_uuid: Uuid,
    #[serde(rename = "StationUUID")]
    pub station_uuid: Uuid,
    #[serde(rename = "FileSize")]
    pub file_size: u64,
    #[serde(rename = "verMajor")]
    pub ver_major: u32,
    #[serde(rename = "verMinor")]
    pub ver_minor: u32,
    #[serde(default, rename = "ForStationsUUID")]
    pub for_stations_uuid: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FileName")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum PacketPayload {
    #[serde(rename = "MVR_JOIN")]
    MvrJoin {
        #[serde(rename = "Provider")]
        provider: String,
        #[serde(rename = "StationName")]
        station_name: String,
        #[serde(rename = "StationUUID")]
        station_uuid: Uuid,
        #[serde(default, rename = "verMajor")]
        ver_major: u32,
        #[serde(default, rename = "verMinor")]
        ver_minor: u32,
        #[serde(default, alias = "Files", rename = "Commits")]
        commits: Vec<MvrCommitArgs>,
    },
    #[serde(rename = "MVR_JOIN_RET")]
    MvrJoinRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
        #[serde(rename = "Provider")]
        provider: String,
        #[serde(rename = "StationName")]
        station_name: String,
        #[serde(rename = "StationUUID")]
        station_uuid: Uuid,
        #[serde(default, rename = "verMajor")]
        ver_major: u32,
        #[serde(default, rename = "verMinor")]
        ver_minor: u32,
        #[serde(default, alias = "Files", rename = "Commits")]
        commits: Vec<MvrCommitArgs>,
    },
    #[serde(rename = "MVR_LEAVE")]
    MvrLeave {
        #[serde(rename = "FromStationUUID")]
        from_station_uuid: Uuid,
    },
    #[serde(rename = "MVR_LEAVE_RET")]
    MvrLeaveRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_COMMIT")]
    MvrCommit {
        #[serde(rename = "FileUUID")]
        file_uuid: Uuid,
        #[serde(rename = "StationUUID")]
        station_uuid: Uuid,
        #[serde(rename = "FileSize")]
        file_size: u64,
        #[serde(rename = "verMajor")]
        ver_major: u32,
        #[serde(rename = "verMinor")]
        ver_minor: u32,
        #[serde(default, rename = "ForStationsUUID")]
        for_stations_uuid: Vec<Uuid>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "FileName")]
        file_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
        comment: Option<String>,
    },
    #[serde(rename = "MVR_COMMIT_RET")]
    MvrCommitRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_REQUEST")]
    MvrRequest {
        #[serde(default, rename = "FileUUID")]
        file_uuid: Option<Uuid>,
        #[serde(default, rename = "FromStationUUID")]
        from_station_uuid: Vec<Uuid>,
    },
    #[serde(rename = "MVR_REQUEST_RET")]
    MvrRequestRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_NEW_SESSION_HOST")]
    MvrNewSessionHost {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceName")]
        service_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceURL")]
        service_url: Option<String>,
    },
    #[serde(rename = "MVR_NEW_SESSION_HOST_RET")]
    MvrNewSessionHostRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(skip, rename = "MVR_FILE")]
    MvrFile(Vec<u8>),
}

impl PacketPayload {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::MvrFile(data) => data.clone(),
            _ => serde_json::to_vec(self).expect("Failed to serialize payload"),
        }
    }

    pub fn decode(bytes: &[u8]) -> Option<(PacketHeader, Self)> {
        let header = PacketHeader::decode(bytes)?;
        let payload_start = PacketHeader::LEN;
        let payload_end = payload_start + header.payload_length as usize;

        if bytes.len() < payload_end {
            return None;
        }

        let payload_bytes = &bytes[payload_start..payload_end];

        let payload = if header.package_type == 1 {
            Self::MvrFile(payload_bytes.to_vec())
        } else {
            serde_json::from_slice(payload_bytes).ok()?
        };

        Some((header, payload))
    }

    pub fn package_type(&self) -> u32 {
        match self {
            Self::MvrFile(_) => 1,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
}

impl Packet {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, io::Error> {
        let mut header_buf = [0u8; PacketHeader::LEN];
        reader.read_exact(&mut header_buf)?;

        let header = PacketHeader::decode(&header_buf)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid header"))?;

        let mut payload_buf = vec![0u8; header.payload_length as usize];
        reader.read_exact(&mut payload_buf)?;

        let payload = if header.package_type == 1 {
            PacketPayload::MvrFile(payload_buf)
        } else {
            serde_json::from_slice(&payload_buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        };

        Ok(Self { header, payload })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        let payload_bytes = self.payload.encode();

        let mut final_header = self.header.clone();
        final_header.payload_length = payload_bytes.len() as u64;
        final_header.package_type = self.payload.package_type();

        writer.write_all(&final_header.encode())?;
        writer.write_all(&payload_bytes)?;
        writer.flush()?;

        Ok(())
    }

    pub fn from_payload(payload: PacketPayload, package_number: u32, package_count: u32) -> Self {
        let payload_bytes = payload.encode();
        let header = PacketHeader::new(payload_bytes.len() as u64, package_number, package_count);
        let mut header = header;
        header.package_type = payload.package_type();
        Self { header, payload }
    }
}
