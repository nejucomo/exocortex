use derive_more::From;
use egui::{Key, Modifiers};

/// Equivalent to [egui::KeyboardShortcut] with additional [From]/[Into] impls
#[derive(Copy, Clone, Debug, From, Eq, PartialEq, Hash)]
pub struct KeyChord {
    key: Key,
    modifiers: Modifiers,
}

impl From<Key> for KeyChord {
    fn from(key: Key) -> Self {
        KeyChord {
            key,
            modifiers: Modifiers::default(),
        }
    }
}
