//! The general `exocortex` [egui] widgets
#![deny(unsafe_code, missing_docs)]

mod card;
mod many;
mod orientation;
mod uiext;

// TODO: Phase out squeeze frame
#[allow(missing_docs)]
pub mod squeeze_frame;

pub mod with;

pub use self::card::{Card, CardBuilder, CardMode, card};
pub use self::many::many;
pub use self::orientation::Orientation;
pub use self::uiext::UiExt;
