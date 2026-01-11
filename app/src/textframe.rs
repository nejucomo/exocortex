mod editor;

use derive_new::new;
use eframe::egui::{Response, Sense, Ui, UiBuilder, Widget};
use egui_commonmark::CommonMarkCache;

use editor::Editor;

use crate::viewer::Viewer;

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct TextFrame<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a mut String,
    editmode: &'a mut bool,
}

impl<'a> Widget for TextFrame<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let avail = ui.available_size();
        let (rect, resp) = ui.allocate_exact_size(avail, Sense::hover());

        let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect));
        if *self.editmode {
            child_ui.add(Editor::new(self.text));
        } else {
            child_ui.add(Viewer::new(self.cmcache, self.text));
        }

        resp
    }
}
