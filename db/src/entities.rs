//! [Entity] types

use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::Entity;
use exocortex_redborm::enumvalue::EnumColumnar;
use redb_derive::Value;

use crate::{ThopId, Timestamp};

impl Entity for ThopV0 {}
impl Entity for ThopSetSynopsisV0 {}
impl Entity for ThopModificationV0 {}

/// The [ThopV0] entity
#[derive(Copy, Clone, Debug, Value)]
pub struct ThopV0;

/// An entity recording a change to synopsis
#[derive(Clone, Debug, From, Into, new, Value)]
pub struct ThopSetSynopsisV0 {
    /// The thop modified
    pub thop: ThopId,
    /// The new synopsis
    pub synopsis: String,
}

/// An entity recording a thop modification
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct ThopModificationV0 {
    /// The thop modified
    pub thop: ThopId,
    /// A time shortly before the db transaction with this modification was committed
    pub time: Timestamp,
    /// The modification enum columnar
    pub enumcol: EnumColumnar,
}
