#![deny(unsafe_code)]

use egui::epaint::MarginF32;
use egui::{Frame, Ui};
use extension_traits::extension;

#[extension(pub trait FrameExt)]
impl Frame {
    fn squeezed_outer_margin(self, ui: &mut Ui) -> Self {
        let avail = ui.available_size();
        let wextra = avail.x - avail.min_elem();
        self.outer_margin(MarginF32::symmetric(wextra, 0.0))
    }
}
