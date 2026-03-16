use std::io;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketHeader {
    pub magic: u32,
    pub version: u32,
    pub number: u32,
    pub count: u32,
    pub r#type: u32,
    pub payload_length: u64,
}

impl PacketHeader {
    pub const LEN: usize = 28;
    pub const MAGIC: u32 = 778682;
    pub const VERSION: u32 = 1;

    pub fn new(payload_length: u64, number: u32, count: u32, r#type: u32) -> Self {
        Self { magic: Self::MAGIC, version: Self::VERSION, number, count, r#type, payload_length }
    }

    pub fn encode(&self) -> [u8; Self::LEN] {
        let mut buf = [0u8; Self::LEN];
        buf[0..4].copy_from_slice(&self.magic.to_be_bytes());
        buf[4..8].copy_from_slice(&self.version.to_be_bytes());
        buf[8..12].copy_from_slice(&self.number.to_be_bytes());
        buf[12..16].copy_from_slice(&self.count.to_be_bytes());
        buf[16..20].copy_from_slice(&self.r#type.to_be_bytes());
        buf[20..28].copy_from_slice(&self.payload_length.to_be_bytes());
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, crate::xchange::Error> {
        if bytes.len() < Self::LEN {
            return Err(crate::xchange::Error::InvalidPacketHeader);
        }

        let read_u32 =
            |start: usize| u32::from_be_bytes(bytes[start..start + 4].try_into().unwrap());
        let read_u64 =
            |start: usize| u64::from_be_bytes(bytes[start..start + 8].try_into().unwrap());

        Ok(Self {
            magic: read_u32(0),
            version: read_u32(4),
            number: read_u32(8),
            count: read_u32(12),
            r#type: read_u32(16),
            payload_length: read_u64(20),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Commit {
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

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "Type")]
pub enum PacketPayload {
    #[serde(rename = "MVR_JOIN")]
    Join {
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
        commits: Vec<Commit>,
    },
    #[serde(rename = "MVR_JOIN_RET")]
    JoinRet {
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
        commits: Vec<Commit>,
    },
    #[serde(rename = "MVR_LEAVE")]
    Leave {
        // NOTE: This alias is cringe, I know. GrandMA3 sends the MVR_LEAVE packet with this field name instead...
        #[serde(rename = "FromStationUUID", alias = "StationUUID")]
        from_station_uuid: Uuid,
    },
    #[serde(rename = "MVR_LEAVE_RET")]
    LeaveRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_COMMIT")]
    Commit {
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
    CommitRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_REQUEST")]
    Request {
        #[serde(default, rename = "FileUUID")]
        file_uuid: Option<Uuid>,
        #[serde(default, rename = "FromStationUUID")]
        from_station_uuid: Vec<Uuid>,
    },
    #[serde(rename = "MVR_REQUEST_RET")]
    RequestRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(rename = "MVR_NEW_SESSION_HOST")]
    NewSessionHost {
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceName")]
        service_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "ServiceURL")]
        service_url: Option<String>,
    },
    #[serde(rename = "MVR_NEW_SESSION_HOST_RET")]
    NewSessionHostRet {
        #[serde(rename = "OK")]
        ok: bool,
        #[serde(default, rename = "Message")]
        message: String,
    },
    #[serde(skip)]
    File(Vec<u8>),
}

impl PacketPayload {
    pub fn r#type(&self) -> u32 {
        match self {
            Self::File(_) => 1,
            _ => 0,
        }
    }

    pub fn serialize_payload(&self) -> Result<Vec<u8>, crate::xchange::Error> {
        match self {
            Self::File(data) => Ok(data.clone()),
            _ => Ok(serde_json::to_vec(self)?),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
}

impl Packet {
    /// Creates a packet from a payload.
    pub fn new(
        payload: PacketPayload,
        number: u32,
        count: u32,
    ) -> Result<Self, crate::xchange::Error> {
        let bytes = payload.serialize_payload()?;
        let header = PacketHeader::new(bytes.len() as u64, number, count, payload.r#type());
        Ok(Self { header, payload })
    }

    pub fn read<R: io::Read>(mut reader: R) -> Result<Self, crate::xchange::Error> {
        let mut header_buf = [0u8; PacketHeader::LEN];
        reader.read_exact(&mut header_buf)?;
        let header = PacketHeader::decode(&header_buf)?;

        let mut payload_buf = vec![0u8; header.payload_length as usize];
        reader.read_exact(&mut payload_buf)?;

        let payload = match header.r#type {
            1 => PacketPayload::File(payload_buf),
            _ => serde_json::from_slice(&payload_buf)?,
        };

        Ok(Self { header, payload })
    }

    pub fn write<W: io::Write>(&self, mut writer: W) -> Result<(), crate::xchange::Error> {
        let payload_bytes = self.payload.serialize_payload()?;

        let mut header = self.header.clone();
        header.payload_length = payload_bytes.len() as u64;
        header.r#type = self.payload.r#type();

        // We have to make sure the bytes are send all at once, without splitting with multiple `write_all`s.
        let header_bytes = header.encode();
        let mut bytes = Vec::with_capacity(header_bytes.len() + payload_bytes.len());
        bytes.extend_from_slice(&header_bytes);
        bytes.extend_from_slice(&payload_bytes);
        writer.write_all(&bytes)?;
        writer.flush()?;

        Ok(())
    }
}
