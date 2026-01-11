use derive_new::new;
use eframe::egui::{Frame, Margin, Response, TextStyle, Ui, Widget};
use eframe::epaint::MarginF32;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[derive(new)]
pub(super) struct Viewer<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a str,
}

impl<'a> Widget for Viewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        CommonMarkViewer::new()
            .show(ui, self.cmcache, self.text)
            .response
    }
}
