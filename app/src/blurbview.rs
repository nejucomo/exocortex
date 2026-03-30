use derive_new::new;
use eframe::egui::{Response, ScrollArea, Sense, Ui, Vec2};
use egui_commonmark::CommonMarkCache;

use crate::blurb::Blurb;
use crate::cmwidget::CommonMarkWidget;

#[derive(Debug, new)]
pub(crate) struct BlurbView<'a> {
    blurbs: &'a [Blurb],
}

impl<'a> CommonMarkWidget for BlurbView<'a> {
    fn ui_with_cmcache(self, ui: &mut Ui, cmcache: &mut CommonMarkCache) -> Response {
        ScrollArea::vertical()
            .show(ui, |ui| {
                let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());

                for blurb in self.blurbs.iter().rev() {
                    r |= blurb.ui_with_cmcache(ui, cmcache);
                }

                r
            })
            .inner
    }
}
