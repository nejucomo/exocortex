mod editor;

use eframe::egui::{Response, Sense, Ui, UiBuilder, Widget};
use egui_commonmark::CommonMarkCache;

use editor::Editor;

use crate::viewer::Viewer;

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

        let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect));
        if self.editmode {
            child_ui.add(Editor::new(&mut self.text));
        } else {
            child_ui.add(Viewer::new(&mut self.cmcache, &self.text));
        }

        resp
    }
}
