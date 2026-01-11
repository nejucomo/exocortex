mod editor;

use derive_new::new;
use eframe::egui::{Response, Sense, Ui, UiBuilder, Widget};
use egui_commonmark::CommonMarkCache;

use editor::Editor;

use crate::viewer::Viewer;

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct ModalEditor<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a mut String,
    editmode: &'a mut bool,
}

impl<'a> Widget for ModalEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.input(|i| {
            use eframe::egui::Key::{Escape, I};

            if *self.editmode {
                if i.key_pressed(Escape) {
                    *self.editmode = false;
                }
            } else if i.key_pressed(I) {
                *self.editmode = true;
            }
        });

        if *self.editmode {
            ui.add(Editor::new(self.text))
        } else {
            ui.add(Viewer::new(self.cmcache, self.text))
        }
    }
}
