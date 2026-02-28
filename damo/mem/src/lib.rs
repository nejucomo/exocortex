//! The `exocortex` ephemeral runtime-memory <u>da</u>ta <u>mo</u>del provider
#![deny(unsafe_code, missing_docs)]

mod card;
mod provider;
mod unknownid;

pub use self::card::MemCard;
pub use self::provider::MemProvider;
pub use self::unknownid::UnknownId;

type Id = u64;
