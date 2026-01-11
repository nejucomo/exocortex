use derive_new::new;
use eframe::egui::{Frame, Margin, Response, TextStyle, Ui, Widget};
use eframe::epaint::MarginF32;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_squeeze_frame::FrameExt as _;

#[derive(new)]
pub(super) struct Viewer<'a> {
    cmcache: &'a mut CommonMarkCache,
    text: &'a str,
}

impl<'a> Widget for Viewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let visuals = ui.style().visuals.clone();

        Frame::NONE
            .stroke({
                let mut stroke = visuals.widgets.active.bg_stroke;
                stroke.color = stroke.color.gamma_multiply(0.05);
                stroke
            })
            .fill(visuals.panel_fill)
            .corner_radius(visuals.widgets.active.corner_radius)
            .squeezed_outer_margin(ui)
            .inner_margin(margin(ui))
            .show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                CommonMarkViewer::new().show(ui, self.cmcache, self.text);
            })
            .response
    }
}

fn margin(ui: &mut Ui) -> MarginF32 {
    let style = TextStyle::Body;
    let space_w = ui.fonts_mut(|f| f.glyph_width(&style.resolve(ui.style()), ' '));
    let line_h = ui.text_style_height(&style);
    MarginF32::symmetric(2. * space_w, 0. * line_h)
}
