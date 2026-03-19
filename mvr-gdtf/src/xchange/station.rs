use std::{net::SocketAddr, time::Instant};

use uuid::Uuid;

use crate::xchange::packet::Commit;

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

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
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
