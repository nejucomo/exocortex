//! [Entity] types

use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::Entity;
use exocortex_redborm::enumvalue::EnumColumnar;
use redb_derive::Value;

use crate::{BlurbId, Timestamp};

impl Entity for BlurbV0 {}
impl Entity for BlurbSetSynopsisV0 {}
impl Entity for BlurbModificationV0 {}

/// The [BlurbV0] entity
#[derive(Copy, Clone, Debug, Value)]
pub struct BlurbV0;

/// An entity recording a change to synopsis
#[derive(Clone, Debug, From, Into, new, Value)]
pub struct BlurbSetSynopsisV0 {
    /// The blurb modified
    pub blurb: BlurbId,
    /// The new synopsis
    pub synopsis: String,
}

/// An entity recording a blurb modification
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct BlurbModificationV0 {
    /// The blurb modified
    pub blurb: BlurbId,
    /// A time shortly before the db transaction with this modification was committed
    pub time: Timestamp,
    /// The modification enum columnar
    pub enumcol: EnumColumnar,
}
