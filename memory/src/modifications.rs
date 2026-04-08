//! Memory modifications

use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::Id;
use exocortex_timestamp::Timestamp;

use crate::Thop;

/// A request to modify a thop
#[derive(Clone, Debug, From, TryInto)]
pub enum ThopModify {
    /// Create a new thop
    Create(Thop),
    /// Update a thop's synopsis
    SetSynopsis(ThopSetSynopsis),
}

/// Parameters for setting a thop's synopsis
#[derive(Clone, Debug, From, new)]
pub struct ThopSetSynopsis {
    /// The thop to modify
    pub thop: Id<Thop>,
    /// The new synopsis text
    pub synopsis: String,
}

/// A record of a completed thop modification
#[derive(Clone, Debug, From, new)]
pub struct ThopModified {
    /// The thop that was modified
    pub thop: Id<Thop>,
    /// The approximate time of the modification
    pub time: Timestamp,
    /// What kind of modification occurred
    pub info: ThopMutation,
}

/// The kind of mutation applied to a thop
#[derive(Clone, Debug, From, new)]
pub enum ThopMutation {
    /// A new thop was created
    Created,
    /// The thop's synopsis was set to the given string
    SetSynopsis(String),
}
