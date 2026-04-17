pub mod channel;
pub mod client;
pub mod codec;
pub mod constants;
pub mod enums;
pub mod frame;
pub mod server;

pub use channel::*;
pub use client::*;
pub use codec::*;
pub use constants::*;
pub use enums::*;
pub use frame::*;
pub use server::*;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

pub const DEFAULT_OMT_PORT: u16 = 6400;

/// OMT network endpoint (host + port + optional display name).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OMTEndpoint {
    pub host: String,
    pub port: u16,
    pub name: Option<String>,
}

impl OMTEndpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self { host: host.into(), port, name: None }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn to_uri(&self) -> String {
        format!("omt://{}:{}", self.host, self.port)
    }

    pub fn to_socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Error)]
pub enum OMTUriError {
    #[error("OMT uri must start with omt://")]
    MissingScheme,
    #[error("OMT uri must include a host")]
    MissingHost,
    #[error("invalid port value")]
    InvalidPort,
}

impl FromStr for OMTEndpoint {
    type Err = OMTUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value.strip_prefix("omt://").ok_or(OMTUriError::MissingScheme)?;
        let mut parts = raw.rsplitn(2, ':');
        let port = parts.next().ok_or(OMTUriError::MissingHost)?;
        let host = parts.next().ok_or(OMTUriError::MissingHost)?;
        if host.trim().is_empty() { return Err(OMTUriError::MissingHost); }
        let port = port.parse::<u16>().map_err(|_| OMTUriError::InvalidPort)?;
        Ok(Self::new(host, port))
    }
}

/// Build a raw OMT metadata frame (header + payload bytes).
pub fn metadata_frame_bytes(xml: &str) -> Vec<u8> {
    let data = xml.as_bytes();
    let data_len = data.len() as i32;
    let mut header = [0u8; 16];
    header[0] = 1;
    header[1] = OMTFrameType::Metadata as u8;
    header[12..16].copy_from_slice(&data_len.to_le_bytes());
    let mut packet = Vec::with_capacity(header.len() + data.len());
    packet.extend_from_slice(&header);
    packet.extend_from_slice(data);
    packet
}
