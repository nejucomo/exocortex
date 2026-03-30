use egui::epaint::MarginF32;
use egui::{Color32, Frame, InnerResponse, Sense, Ui, UiBuilder};
use extension_traits::extension;

const CORNER_RADIUS: f32 = 6.0;
const INNER_MARGIN: MarginF32 = MarginF32::symmetric(6.0, 2.0);
const STROKE_GAMMA: f32 = 0.03;
const FILL_BLEND: Color32 = Color32::LIGHT_BLUE;
const FILL_GAMMA: f32 = 0.01;

#[extension(pub trait UiSqueezeExt)]
impl Ui {
    fn within_widgets<F, R>(&mut self, f: F) -> InnerResponse<R>
    where
        F: FnOnce(&mut Ui) -> R,
    {
        let visuals = &self.style().visuals;

        Frame::NONE
            .stroke({
                let mut stroke = visuals.widgets.active.bg_stroke;
                stroke.color = stroke.color.gamma_multiply(STROKE_GAMMA);
                stroke
            })
            .corner_radius(CORNER_RADIUS)
            .fill(
                visuals
                    .panel_fill
                    .blend(FILL_BLEND.gamma_multiply(FILL_GAMMA)),
            )
            .corner_radius(visuals.widgets.active.corner_radius)
            .squeezed_outer_margin(self)
            .inner_margin(INNER_MARGIN)
            .show(self, |ui| {
                let avail = ui.available_size();
                let (rect, _resp) = ui.allocate_exact_size(avail, Sense::hover());

                let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect));
                child_ui.set_min_size(child_ui.available_size());
                f(&mut child_ui)
            })
    }
}

#[extension(pub trait FrameSqueezeExt)]
impl Frame {
    fn squeezed_outer_margin(self, ui: &mut Ui) -> Self {
        let avail = ui.available_size();
        let wextra = avail.x - avail.min_elem();
        self.outer_margin(MarginF32::symmetric(wextra, 0.0))
    }
}
