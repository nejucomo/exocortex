//! The `exocortex` <u>da</u>ta <u>mo</u>del
#![deny(unsafe_code, missing_docs)]
#![allow(missing_docs)] // Expedient

mod canopy;
mod mem;
mod multi;
mod provider;

/// The local-identifier type for cards
pub type Id = u64;

pub mod errors;
pub use self::canopy::CanopyProvider;
pub use self::mem::MemProvider;
pub use self::multi::MultiProvider;
pub use self::provider::Provider;
