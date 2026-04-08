#![deny(unsafe_code, missing_docs)]
#![allow(missing_docs)] // FIXME: expedient

mod entities;
mod error;
mod redhandler;
mod redmem;

pub use self::error::{RedError, RedResult};
pub use self::redmem::RedMem;
