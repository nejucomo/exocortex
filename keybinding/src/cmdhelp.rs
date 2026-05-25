use derive_new::new;
use egui::{Response, Ui, Widget};

use crate::KeyChord;

/// A short help synopsis of a command along with the [KeyChord] sequence that invokes it
#[derive(Debug, new)]
pub struct KeyCommandHelp {
    /// The [KeyChord]s which invoke this command
    pub chords: Vec<KeyChord>,
    /// A short help synopsis for the command
    pub help: &'static str,
}

impl Widget for &KeyCommandHelp {
    fn ui(self, ui: &mut Ui) -> Response {
        let r1 = ui
            .horizontal(|ui| {
                for chord in self.chords.iter().copied() {
                    ui.add(chord);
                }
            })
            .response;

        let r2 = ui.label(self.help);

        r1 | r2
    }
}
