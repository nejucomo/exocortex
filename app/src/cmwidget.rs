use eframe::egui::{Response, Ui};
use egui_commonmark::CommonMarkCache;

pub(crate) trait CommonMarkWidget {
    fn ui_with_cmcache(self, ui: &mut Ui, cmcache: &mut CommonMarkCache) -> Response;
}
