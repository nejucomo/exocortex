//! [Entity] types

use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::Entity;
use exocortex_redborm::enumvalue::EnumColumnar;
use redb_derive::Value;

use crate::{CardId, Timestamp};

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
    pub card: CardId,
    /// The new synopsis
    pub synopsis: String,
}

/// An entity recording a card modification
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct CardModificationV0 {
    /// The card modified
    pub card: CardId,
    /// A time shortly before the db transaction with this modification was committed
    pub time: Timestamp,
    /// The modification enum columnar
    pub enumcol: EnumColumnar,
}
