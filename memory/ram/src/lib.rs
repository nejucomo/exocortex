//! The `exocortex-memory-ram` crate: an in-memory (non-persistent) [`Provider`](exocortex_memory::Provider) implementation.
#![deny(unsafe_code, missing_docs)]

mod error;
mod rammem;

pub use self::error::{RamError, RamResult};
pub use self::rammem::RamMem;
