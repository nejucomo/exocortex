use derive_new::new;
use eframe::egui::{Response, Ui, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[allow(dead_code)]
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
