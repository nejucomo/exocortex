//! channel helpers for [std::sync::mpsc] and [std::thread]
#![deny(unsafe_code, missing_docs)]
#![allow(missing_docs)] // expedient

mod child;
mod error;
mod interface;
mod service;
mod svcinner;

pub use self::error::{ReqRepError, ReqRepRes};
pub use self::service::ThreadService;
