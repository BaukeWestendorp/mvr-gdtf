use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketHeader {
    pub header: u32,
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

    fn new(payload_length: u64, number: u32, count: u32, r#type: u32) -> Self {
        Self { header: Self::MAGIC, version: Self::VERSION, number, count, r#type, payload_length }
    }

    fn encode_into(&self, buf: &mut impl BufMut) {
        buf.put_u32(self.header);
        buf.put_u32(self.version);
        buf.put_u32(self.number);
        buf.put_u32(self.count);
        buf.put_u32(self.r#type);
        buf.put_u64(self.payload_length);
    }

    fn decode_from(buf: &mut impl Buf) -> Result<Self, crate::xchange::Error> {
        if buf.remaining() < Self::LEN {
            return Err(crate::xchange::Error::InvalidPacketHeader);
        }
        Ok(Self {
            header: buf.get_u32(),
            version: buf.get_u32(),
            number: buf.get_u32(),
            count: buf.get_u32(),
            r#type: buf.get_u32(),
            payload_length: buf.get_u64(),
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
        // GrandMA3 sends this field as "StationUUID" instead — alias handles both.
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
    /// Wire type discriminant: 1 = raw bytes, 0 = JSON.
    fn wire_type(&self) -> u32 {
        match self {
            Self::File(_) => 1,
            _ => 0,
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, crate::xchange::Error> {
        match self {
            Self::File(data) => Ok(data.clone()),
            _ => Ok(serde_json::to_vec(self)?),
        }
    }

    fn deserialize(wire_type: u32, bytes: &[u8]) -> Result<Self, crate::xchange::Error> {
        match wire_type {
            1 => Ok(Self::File(bytes.to_vec())),
            _ => Ok(serde_json::from_slice(bytes)?),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
}

impl Packet {
    pub fn new(
        payload: PacketPayload,
        number: u32,
        count: u32,
    ) -> Result<Self, crate::xchange::Error> {
        let bytes = payload.serialize()?;
        let header = PacketHeader::new(bytes.len() as u64, number, count, payload.wire_type());
        Ok(Self { header, payload })
    }
}

/// A tokio-util `Codec` that frames `Packet`s over a byte stream.
///
/// The wire format is unchanged:
///   [28-byte big-endian header][payload_length bytes of payload]
///
/// Usage:
/// ```rust
/// use tokio_util::codec::Framed;
/// use futures::{SinkExt, StreamExt};
///
/// let framed = Framed::new(tcp_stream, PacketCodec);
/// let (mut sink, mut stream) = framed.split();
///
/// sink.send(packet).await?;
/// let reply: Packet = stream.next().await.unwrap()?;
/// ```
pub struct PacketCodec;

//
// The decoder is a simple two-phase state machine driven by what is already
// in the `BytesMut` buffer:
//
//   Phase 1 – wait for 28 header bytes, then peek at payload_length.
//   Phase 2 – wait for header + payload_length bytes, then consume and parse.
//
// Returning `Ok(None)` tells tokio-util "not enough data yet; call me again
// when more bytes arrive." tokio-util handles all the buffering for us.

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = crate::xchange::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Packet>, Self::Error> {
        // Phase 1: do we have a full header?
        if src.len() < PacketHeader::LEN {
            src.reserve(PacketHeader::LEN - src.len());
            return Ok(None);
        }

        // Peek at the header without advancing the cursor yet — we need to
        // keep the bytes around in case the payload hasn't arrived yet.
        let header = {
            let mut peek = &src[..PacketHeader::LEN];
            PacketHeader::decode_from(&mut peek)?
        };

        let frame_len = PacketHeader::LEN + header.payload_length as usize;

        // Phase 2: do we have the full frame (header + payload)?
        if src.len() < frame_len {
            src.reserve(frame_len - src.len());
            return Ok(None);
        }

        // We have a complete frame — advance past the header and consume.
        src.advance(PacketHeader::LEN);
        let payload_bytes = src.split_to(header.payload_length as usize);
        let payload = PacketPayload::deserialize(header.r#type, &payload_bytes)?;

        Ok(Some(Packet { header, payload }))
    }
}

impl Encoder<Packet> for PacketCodec {
    type Error = crate::xchange::Error;

    fn encode(&mut self, packet: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload_bytes = packet.payload.serialize()?;

        // Rebuild the header so payload_length and type are always consistent,
        // regardless of what the caller put in `packet.header`.
        let header = PacketHeader {
            payload_length: payload_bytes.len() as u64,
            r#type: packet.payload.wire_type(),
            ..packet.header
        };

        dst.reserve(PacketHeader::LEN + payload_bytes.len());
        header.encode_into(dst);
        dst.put_slice(&payload_bytes);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    fn roundtrip(packet: Packet) -> Packet {
        let mut buf = BytesMut::new();
        PacketCodec.encode(packet, &mut buf).expect("encode");
        PacketCodec.decode(&mut buf).expect("decode").expect("complete frame")
    }

    #[test]
    fn join_roundtrip() {
        let uuid = Uuid::new_v4();
        let packet = Packet::new(
            PacketPayload::Join {
                provider: "test".into(),
                station_name: "station-1".into(),
                station_uuid: uuid,
                ver_major: 1,
                ver_minor: 2,
                commits: vec![],
            },
            0,
            1,
        )
        .unwrap();

        let decoded = roundtrip(packet);
        assert!(matches!(
            decoded.payload,
            PacketPayload::Join { station_uuid, .. } if station_uuid == uuid
        ));
    }

    #[test]
    fn file_roundtrip() {
        let data = vec![1u8, 2, 3, 4, 5];
        let packet = Packet::new(PacketPayload::File(data.clone()), 0, 1).unwrap();
        let decoded = roundtrip(packet);
        assert_eq!(decoded.payload, PacketPayload::File(data));
    }

    #[test]
    fn partial_decode_returns_none() {
        let packet =
            Packet::new(PacketPayload::LeaveRet { ok: true, message: String::new() }, 0, 1)
                .unwrap();
        let mut buf = BytesMut::new();
        PacketCodec.encode(packet, &mut buf).unwrap();

        // Feed only half the bytes.
        let half = buf.split_to(buf.len() / 2);
        let mut partial = half;
        let result = PacketCodec.decode(&mut partial).unwrap();
        assert!(result.is_none(), "should return None on partial frame");
    }
}
