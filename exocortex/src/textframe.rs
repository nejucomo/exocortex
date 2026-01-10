mod editor;
mod viewer;

use eframe::egui::{Response, Sense, Ui, Widget};
use egui_commonmark::CommonMarkCache;

use editor::Editor;
use viewer::Viewer;

#[derive(Default)]
pub(crate) struct TextFrame {
    cmcache: CommonMarkCache,
    text: String,
    pub(crate) editmode: bool,
}

impl Widget for &mut TextFrame {
    fn ui(self, ui: &mut Ui) -> Response {
        let avail = ui.available_size();
        let (rect, resp) = ui.allocate_exact_size(avail, Sense::hover());

        #[allow(deprecated)]
        ui.allocate_ui_at_rect(rect, |ui| {
            if self.editmode {
                ui.add(Editor::new(&mut self.text))
            } else {
                ui.add(Viewer::new(&mut self.cmcache, &mut self.text))
            };
        });

        resp
    }
}
