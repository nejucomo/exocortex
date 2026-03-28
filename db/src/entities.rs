//! [Entity](exocortex_redborm::Entity) types

use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::enumvalue::EnumColumnar;
use exocortex_redborm::{Entity, Id};
use redb_derive::Value;

use crate::Timestamp;

impl Entity for CardV0 {}
impl Entity for CardSetSynopsisV0 {}
impl Entity for CardModificationV0 {}

/// The [CardV0] entity
#[derive(Copy, Clone, Debug, Value)]
pub struct CardV0;

/// An entity recording a change to synopsis
#[derive(Clone, Debug, From, Into, new, Value)]
pub struct CardSetSynopsisV0 {
    /// The card modified
    pub card: Id<CardV0>,
    /// The new synopsis
    pub synopsis: String,
}

/// An entity recording a card modification
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct CardModificationV0 {
    /// The card modified
    pub card: Id<CardV0>,
    /// A time shortly before the db transaction with this modification was committed
    pub time: Timestamp,
    /// The modification enum columnar
    pub enumcol: EnumColumnar,
}
