//! The `exocortex` <u>da</u>ta <u>mo</u>del
#![deny(unsafe_code, missing_docs)]

mod mem;
mod provider;

/// The local-identifier type for cards
pub type Id = u64;

pub use self::mem::{MemCard, MemProvider, UnknownId};
pub use self::provider::{Card, Provider, ProviderErrors};
