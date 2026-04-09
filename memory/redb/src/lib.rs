//! The `exocortex-memory-redb` crate: a [`redb`]-backed [`Provider`](exocortex_memory::Provider) implementation.
#![deny(unsafe_code, missing_docs)]

mod entities;
mod error;
mod redhandler;
mod redmem;

pub use self::error::{RedError, RedResult};
pub use self::redmem::RedMem;
