//! channel helpers for [std::sync::mpsc] and [std::thread]
#![deny(unsafe_code, missing_docs)]
#![allow(missing_docs)] // expedient

mod channel;
mod error;
mod interface;
mod pair;
mod service;
mod svcinner;

pub(crate) use self::channel::Channel;
pub(crate) use self::interface::Interface;
pub(crate) use self::pair::InterfacePair;
pub(crate) use self::svcinner::SvcInner;

pub use self::error::{ReqRepError, ReqRepRes};
pub use self::service::ThreadService;
