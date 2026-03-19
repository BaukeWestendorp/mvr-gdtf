use std::io;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("mDNS error: {0}")]
    MDns(#[from] mdns_sd::Error),

    #[error("Timed out")]
    Timeout,
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("The service has been shutdown")]
    Shutdown,

    #[error("Packet serialization error: {0}")]
    PacketSerialization(#[from] serde_json::Error),
    #[error("Invalid header magic or length")]
    InvalidPacketHeader,
    #[error("Received an unexpected packet")]
    UnexpectedPacket,
    #[error("Invalid packet: {message}")]
    InvalidPacket { message: String },

    #[error("Station with uuid {uuid} not found")]
    StationNotFound { uuid: Uuid },
}
