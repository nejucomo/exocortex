use derive_more::From;
use eframe::egui::{InputState, Ui, ViewportCommand};

use self::CommandKey::*;

#[derive(Debug, From)]
pub(crate) enum CommandKey {
    Viewport(ViewportCommand),
    OpenNewJournal,
}

impl CommandKey {
    pub(crate) fn get(ui: &Ui) -> Option<Self> {
        ui.input(|i| {
            use ViewportCommand::{Close, Fullscreen};
            use eframe::egui::Key::{Escape, F, J};

            if !i.modifiers.command {
                None
            } else if i.key_pressed(Escape) {
                Some(Viewport(Close))
            } else if i.key_pressed(F) {
                Some(Viewport(Fullscreen(
                    !i.viewport().fullscreen.unwrap_or_default(),
                )))
            } else if i.key_pressed(J) {
                Some(OpenNewJournal)
            } else {
                None
            }
        })
    }
}
