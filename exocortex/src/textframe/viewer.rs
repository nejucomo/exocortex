use derive_new::new;
use eframe::egui::{Color32, Frame, Response, Stroke, Ui, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[derive(new)]
pub(super) struct Viewer<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a mut String,
}

impl<'a> Widget for Viewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        Frame::NONE
            .stroke(Stroke::new(1.0, Color32::RED))
            .show(ui, |ui| {
                CommonMarkViewer::new().show_mut(ui, self.cmcache, self.text);
            })
            .response
    }
}
