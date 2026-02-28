//! The `exocortex` <u>da</u>ta <u>mo</u>del
#![deny(unsafe_code, missing_docs)]

mod id;
mod memprovider;
mod provider;
mod query;
mod update;

pub mod queries;
pub mod updates;

pub use self::id::Id;
pub use self::memprovider::MemoryProvider;
pub use self::provider::{Provider, ProviderBase};
pub use self::query::Query;
pub use self::update::Update;
