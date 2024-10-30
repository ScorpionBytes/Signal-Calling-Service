//
// Copyright 2023 Signal Messenger, LLC
// SPDX-License-Identifier: AGPL-3.0-only
//

use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Clone, Debug, Eq, EnumString, EnumIter, PartialEq, Copy, Hash, Serialize, Deserialize)]
pub enum CallType {
    Adhoc,
    GroupV2,
}

impl CallType {
    pub fn as_tag(&self) -> &'static str {
        match self {
            CallType::Adhoc => "call-type:Adhoc",
            CallType::GroupV2 => "call-type:GroupV2",
        }
    }
}

/// A wrapper around a u32 with the 4 LSBs set to 0.
/// Uniquely identifies a client within a call (scoped to the call era).
#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash, PartialOrd, Ord)]
pub struct DemuxId(u32);

impl DemuxId {
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// For testing
    pub const fn from_const(raw: u32) -> Self {
        assert!(raw & 0b1111 == 0, "lowest 4 bits must be clear");
        Self(raw)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid demux ID: {0} ({0:#x})")]
pub struct InvalidDemuxIdError(u32);

impl TryFrom<u32> for DemuxId {
    type Error = InvalidDemuxIdError;
    fn try_from(demux_id: u32) -> Result<Self, InvalidDemuxIdError> {
        if demux_id & 0b1111 == 0 {
            Ok(Self(demux_id))
        } else {
            Err(InvalidDemuxIdError(demux_id))
        }
    }
}

pub const DUMMY_DEMUX_ID: DemuxId = DemuxId(0);

impl From<DemuxId> for u32 {
    fn from(demux_id: DemuxId) -> u32 {
        demux_id.0
    }
}

#[derive(Clone, Deserialize, Hash, Serialize, Eq, PartialEq)]
pub struct RoomId(String);

impl From<String> for RoomId {
    fn from(room_id_string: String) -> Self {
        Self(room_id_string)
    }
}

impl From<&str> for RoomId {
    fn from(room_id: &str) -> Self {
        Self(room_id.to_string())
    }
}

impl From<RoomId> for String {
    fn from(room_id: RoomId) -> Self {
        room_id.0
    }
}

impl AsRef<str> for RoomId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Implement Display for RoomId to redact most of the string.
impl fmt::Display for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

/// Implement Debug for RoomId to redact most of the string.
impl fmt::Debug for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}
