//! channel helpers for [std::sync::mpsc] and [std::thread]
#![deny(unsafe_code, missing_docs)]

mod channel;
mod interface;
mod pair;

pub mod reqrep;
pub use self::channel::Channel;
pub use self::interface::Interface;
pub use self::pair::InterfacePair;
