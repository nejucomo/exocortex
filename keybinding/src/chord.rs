use egui::{Key, ModifierNames, Modifiers, Response, RichText, Ui, Widget};

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

impl Widget for KeyChord {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(
            RichText::new(format!(
                "{} {}",
                ModifierNames::SYMBOLS.format(&self.modifiers, ui.ctx().os().is_mac()),
                self.key.symbol_or_name()
            ))
            .code(),
        )
    }
}
