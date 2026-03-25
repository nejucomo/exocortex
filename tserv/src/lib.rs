//! Run a request -> reply child [std::thread] via [std::sync::mpsc] channels
#![deny(unsafe_code, missing_docs)]

mod child;
mod interface;
mod service;
mod svcinner;

pub use self::service::ThreadService;
