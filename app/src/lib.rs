#![deny(unsafe_code)]

mod aggregate;
mod app;
mod cardview;
mod command;
mod dbgexpr;
mod logview;
mod modaleditor;
mod prepop;
mod run;
mod viewer;

pub mod cliopts;
pub use self::run::run;
