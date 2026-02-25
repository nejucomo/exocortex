use egui::{Key, Modifiers};

/// Equivalent to [egui::KeyboardShortcut] with additional [From]/[Into] impls
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

impl From<(Key, Modifiers)> for KeyChord {
    fn from((key, modifiers): (Key, Modifiers)) -> Self {
        KeyChord {
            key,
            modifiers: Modifiers {
                mac_cmd: false,
                ..modifiers
            },
        }
    }
}
