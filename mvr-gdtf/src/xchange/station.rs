use std::{net::SocketAddr, time::Instant};

use uuid::Uuid;

use crate::xchange::packet::Commit;

/// Information about a MVR-xchange station.
#[derive(Debug, Clone, PartialEq)]
pub struct StationInfo {
    uuid: Uuid,
    name: String,
    pub(crate) address: SocketAddr,
    provider: String,
    ver_major: u32,
    ver_minor: u32,
    pub(crate) commits: Vec<Commit>,
    pub(crate) last_seen: Instant,
}

impl StationInfo {
    pub(crate) fn new(
        uuid: Uuid,
        name: String,
        address: SocketAddr,
        provider: String,
        ver_major: u32,
        ver_minor: u32,
        commits: Vec<Commit>,
    ) -> Self {
        Self {
            uuid,
            name,
            address,
            provider,
            ver_major,
            ver_minor,
            commits,
            last_seen: Instant::now(),
        }
    }

    /// Returns the station's UUID.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Returns the station's human-readable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the provider/application name of the station.
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns the major MVR file version supported by the station.
    pub fn ver_major(&self) -> u32 {
        self.ver_major
    }

    /// Returns the minor MVR file version supported by the station.
    pub fn ver_minor(&self) -> u32 {
        self.ver_minor
    }

    /// Returns the commit metadata last advertised by the station.
    pub fn commits(&self) -> &[Commit] {
        &self.commits
    }
}
