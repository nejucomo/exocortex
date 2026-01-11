#![deny(unsafe_code)]

use egui::epaint::MarginF32;
use egui::{Frame, InnerResponse, TextStyle, Ui};
use extension_traits::extension;

#[extension(pub trait UiExt)]
impl Ui {
    fn within_squeeze_frame<F, R>(&mut self, f: F) -> InnerResponse<R>
    where
        F: FnOnce(&mut Ui) -> R,
    {
        let visuals = self.style().visuals.clone();

        Frame::NONE
            .stroke({
                let mut stroke = visuals.widgets.active.bg_stroke;
                stroke.color = stroke.color.gamma_multiply(0.05);
                stroke
            })
            .fill(visuals.panel_fill)
            .corner_radius(visuals.widgets.active.corner_radius)
            .squeezed_outer_margin(self)
            .inner_margin(margin(self))
            .show(self, |ui| {
                ui.set_min_size(ui.available_size());
                f(ui)
            })
    }
}

#[extension(pub trait FrameExt)]
impl Frame {
    fn squeezed_outer_margin(self, ui: &mut Ui) -> Self {
        let avail = ui.available_size();
        let wextra = avail.x - avail.min_elem();
        self.outer_margin(MarginF32::symmetric(wextra, 0.0))
    }
}

fn margin(ui: &mut Ui) -> MarginF32 {
    let style = TextStyle::Body;
    let space_w = ui.fonts_mut(|f| f.glyph_width(&style.resolve(ui.style()), ' '));
    let line_h = ui.text_style_height(&style);
    MarginF32::symmetric(2. * space_w, 0. * line_h)
}
