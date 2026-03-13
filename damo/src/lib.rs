//! The `exocortex` <u>da</u>ta <u>mo</u>del
#![deny(unsafe_code, missing_docs)]
#![allow(missing_docs)] // Expedient

mod error;
mod mem;
mod multi;
mod provider;
mod red;

/// The local-identifier type for cards
pub type Id = u64;

pub use self::error::{DamoError, DamoResult};
pub use self::mem::MemProvider;
pub use self::multi::MultiProvider;
pub use self::provider::Provider;
pub use self::red::RedProvider;
