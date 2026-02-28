//! The `exocortex` <u>da</u>ta <u>mo</u>del
#![deny(unsafe_code, missing_docs)]

mod id;
mod provider;
mod query;
mod update;

pub use self::id::Id;
pub use self::provider::{Provider, ProviderBase};
pub use self::query::Query;
pub use self::update::Update;
