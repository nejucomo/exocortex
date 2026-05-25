use derive_new::new;
use egui::{Frame, Grid, Response, RichText, Ui, Widget};

use crate::KeyCommandHelp;

/// A display frame of the keyboard shortcuts
#[derive(Debug, new)]
pub struct ShortcutDisplay<'a> {
    helps: &'a [KeyCommandHelp],
}

impl<'a> Widget for ShortcutDisplay<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(RichText::new("Keyboard Shortcuts").heading());

        Frame::group(ui.style())
            .show(ui, |ui| ui.add(KeyMapDisplay(self.helps)))
            .response
    }
}

struct KeyMapDisplay<'a>(&'a [KeyCommandHelp]);

impl<'a> Widget for KeyMapDisplay<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        Grid::new("FIXME KeyMapDisplay: in the future we want multiple keymap displays for different contexts")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            for help in self.0 {
                ui.add(help);
                ui.end_row();
            }
        }).response
    }
}
