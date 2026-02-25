//! Keyboard shortcut mapping and input for `exocortex`
#![deny(unsafe_code, missing_docs)]

mod keymap;
mod shortcuts;

pub use self::shortcuts::{HandleKey, ShortcutState, UnknownSequence};
