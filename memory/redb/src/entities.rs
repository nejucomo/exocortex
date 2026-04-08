//! [Entity] types

use derive_more::{From, Into};
use derive_new::new;
use exocortex_lid::Id;
use exocortex_memory as mem;
use exocortex_redborm::Entity;
use exocortex_redborm::enumvalue::EnumColumnar;
use exocortex_timestamp::Timestamp;
use redb_derive::Value;

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
    pub thop: Id<mem::Thop>,
    /// The new synopsis
    ///
    /// # TODO
    ///
    /// Figure out how to modify `redborm` to enable this being `&str` to remove an excessive copy on db write.
    pub synopsis: String,
}

/// An entity recording a thop modification
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct ThopModificationV0 {
    /// The thop modified
    pub thop: Id<mem::Thop>,
    /// A time shortly before the db transaction with this modification was committed
    pub time: Timestamp,
    /// The modification enum columnar
    pub enumcol: EnumColumnar,
}

impl From<&mem::modifications::ThopSetSynopsis> for ThopSetSynopsisV0 {
    fn from(tss: &mem::modifications::ThopSetSynopsis) -> Self {
        Self {
            thop: tss.thop,
            // TODO: if we could redefine `Self::synopsis` as `&str` we can remove a `String::clone`
            synopsis: tss.synopsis.clone(),
        }
    }
}
