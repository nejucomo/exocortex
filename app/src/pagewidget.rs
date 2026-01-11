use eframe::egui::{Response, Ui, Widget as _};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_page::Page;

use crate::viewer::Viewer;

pub(crate) trait PageWidget {
    fn show_page(&mut self, ui: &mut Ui, cmc: &mut CommonMarkCache) -> Response;
}

impl PageWidget for Page {
    fn show_page(&mut self, ui: &mut Ui, cmc: &mut CommonMarkCache) -> Response {
        use Page::*;

        ui.set_min_size(ui.available_size());

        match self {
            Static(text) => Viewer::new(cmc, text).ui(ui),
        }
    }
}
