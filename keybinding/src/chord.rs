use egui::{Color32, Frame, Key, Margin, ModifierNames, Modifiers, Response, RichText, Ui, Widget};
use tap::Tap as _;

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
        let crad: f32 = ui.text_style_height(&egui::TextStyle::Body) / 2.0;

        Frame::group(ui.style())
            .tap_mut(|f| {
                let c = Color32::BLUE;
                f.fill = f.fill.lerp_to_gamma(c, 0.1);
                f.stroke.color = f.stroke.color.lerp_to_gamma(c, 0.2);
                f.stroke.width *= 0.5;
            })
            .corner_radius(crad)
            .inner_margin(Margin::same((crad / 2.0) as i8))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!(
                        "{} {}",
                        ModifierNames::SYMBOLS.format(&self.modifiers, ui.ctx().os().is_mac()),
                        self.key.symbol_or_name()
                    ))
                    .monospace(),
                )
            })
            .response
    }
}
