//! Memory modifications

use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::{Id, IdMap, IdMapResult};
use exocortex_thop::Thop;
use exocortex_timestamp::Timestamp;

/// A request to modify a thop
#[derive(Clone, Debug, From, TryInto)]
pub enum ThopModify {
    /// Create a new thop
    Create(ThopCreate),
    /// Update a thop's synopsis
    SetSynopsis(ThopSetSynopsis),
}

/// A request to create a new [Thop]
#[derive(Copy, Clone, Debug)]
pub struct ThopCreate;

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

impl ThopModified {
    /// Apply this modification to an in-memory view of [Thop]s
    pub fn modify_thop_map(&self, thopmap: &mut IdMap<Thop>) -> IdMapResult<(), Thop> {
        use ThopMutation::*;

        let time = self.time.clone();

        match &self.info {
            Created => thopmap.insert_new(self.thop, Thop::new(time.clone(), time, "")),
            SetSynopsis(syn) => thopmap.get_mut(self.thop).map(|thop| {
                thop.mtime = time;
                thop.synopsis = syn.clone();
            }),
        }
    }
}

/// The kind of mutation applied to a thop
#[derive(Clone, Debug, From, new)]
pub enum ThopMutation {
    /// A new thop was created
    Created,
    /// The thop's synopsis was set to the given string
    SetSynopsis(String),
}
