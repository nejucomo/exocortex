//! The general `exocortex` [egui] widgets
#![deny(unsafe_code, missing_docs)]

mod card;
mod uiext;

// TODO: Phase out squeeze frame
#[allow(missing_docs)]
pub mod squeeze_frame;

pub use self::card::card;
