use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("mDNS error: {0}")]
    MDns(#[from] mdns_sd::Error),

    #[error("Packet serialization error: {0}")]
    PacketSerialization(#[from] serde_json::Error),
    #[error("Invalid header magic or length")]
    InvalidPacketHeader,

    #[error("Sender channel closed")]
    ChannelClosed,

    #[error("Station UUID is invalid")]
    StationUuidInvalid,
    #[error("Station UUID is missing")]
    StationUuidMissing,
    #[error("Station name is missing")]
    StationNameMissing,

    #[error("Station not found")]
    StationNotFound,
}
