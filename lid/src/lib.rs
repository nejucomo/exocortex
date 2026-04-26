#![doc = include_str!("id-synopsis.md")]
#![deny(unsafe_code, missing_docs)]

mod id;
mod idmap;
mod withid;

#[cfg(feature = "redb")]
mod redbimpls;

pub use self::id::Id;
pub use self::idmap::{IdMap, IdMapError, IdMapResult};
pub use self::withid::{ValueWithId, WithId};
