#![doc = include_str!("id-synopsis.md")]
#![deny(unsafe_code, missing_docs)]

mod id;
mod withid;

#[cfg(feature = "redb")]
mod redbimpls;

pub use self::id::Id;
pub use self::withid::{ValueWithId, WithId};
