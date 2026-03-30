#![deny(unsafe_code)]

mod app;
mod blurb;
mod blurbview;
mod cmwidget;
mod command;
mod dbgexpr;
mod dbglayout;
mod logview;
mod modaleditor;
mod prepop;
mod run;
mod viewer;

pub mod cliopts;
pub use self::run::run;
