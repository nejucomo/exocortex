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
        let avail = ui.available_size();

        Frame::NONE
            .stroke(Stroke::new(1.0, Color32::RED))
            .show(ui, |ui| {
                ui.set_min_size(avail);
                CommonMarkViewer::new().show_mut(ui, self.cmcache, self.text);
            })
            .response
    }
}
