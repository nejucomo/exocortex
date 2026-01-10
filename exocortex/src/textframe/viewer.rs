use derive_new::new;
use eframe::egui::{Frame, Response, Ui, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[derive(new)]
pub(super) struct Viewer<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a mut String,
}

impl<'a> Widget for Viewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let avail = ui.available_size();
        let visuals = ui.style().visuals.clone();

        Frame::NONE
            .stroke(visuals.widgets.active.bg_stroke)
            .fill(visuals.panel_fill)
            .corner_radius(visuals.widgets.active.corner_radius)
            .show(ui, |ui| {
                ui.set_min_size(avail);
                CommonMarkViewer::new().show_mut(ui, self.cmcache, self.text);
            })
            .response
    }
}
