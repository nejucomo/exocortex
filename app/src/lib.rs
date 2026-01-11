#![deny(unsafe_code)]
#![allow(unused_imports)]

mod app;
mod dbgexpr;
mod pagewidget;
mod run;
mod textframe;
mod viewer;

pub mod cliopts;
pub use self::run::run;
