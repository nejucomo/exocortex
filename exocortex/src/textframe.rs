mod editor;
mod viewer;

use eframe::egui::{Response, Ui, Widget};
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
        if self.editmode {
            ui.add(Editor::new(&mut self.text))
        } else {
            ui.add(Viewer::new(&mut self.cmcache, &mut self.text))
        }
    }
}
