#![warn(missing_docs)]

//! MVR-xchange implementation.
//!
//! This module contains the networking and protocol implementation for
//! discovering peers over mDNS and exchanging MVR files over TCP.
//!
//! The primary entry point is [`Service`], which runs a background task and
//! exposes a cloneable handle for interacting with the station.

mod error;
mod packet;
mod service;
mod station;

pub use error::*;
pub use service::*;
pub use station::*;
