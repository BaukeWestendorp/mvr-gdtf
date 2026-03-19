use std::io;

use uuid::Uuid;

/// Error type used by the MVR-xchange implementation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An underlying I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// An error originating from mDNS service discovery/registration.
    #[error("mDNS error: {0}")]
    MDns(#[from] mdns_sd::Error),

    /// A request timed out while waiting for a response.
    #[error("Timed out")]
    Timeout,

    /// The remote side closed the connection unexpectedly.
    #[error("Connection closed")]
    ConnectionClosed,

    /// The background service task has been shut down.
    #[error("The service has been shutdown")]
    Shutdown,

    /// JSON serialisation/deserialisation failed for a packet payload.
    #[error("Packet serialization error: {0}")]
    PacketSerialization(#[from] serde_json::Error),

    /// The packet header is invalid (bad magic/length, or too short).
    #[error("Invalid header magic or length")]
    InvalidPacketHeader,

    /// A packet was received that is not valid in the current context/state.
    #[error("Received an unexpected packet")]
    UnexpectedPacket,

    /// The received packet was invalid.
    #[error("Invalid packet: {message}")]
    InvalidPacket {
        /// Information about the error.
        message: String,
    },

    /// A station with the given UUID was not found.
    #[error("Station with uuid {uuid} not found")]
    StationNotFound {
        /// UUID of the missing station.
        uuid: Uuid,
    },
}
