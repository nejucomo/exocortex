use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2};
use egui_commonmark::CommonMarkCache;

use crate::blurb::Blurb;
use crate::cmwidget::CommonMarkWidget;

#[derive(Debug, new)]
pub(crate) struct BlurbView<'a> {
    blurbs: &'a [Blurb],
}

impl<'a> CommonMarkWidget for BlurbView<'a> {
    fn ui_with_cmcache(self, ui: &mut Ui, cmcache: &mut CommonMarkCache) -> Response {
        let mut resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for blurb in self.blurbs.iter().rev() {
                resp |= blurb.ui_with_cmcache(ui, cmcache);

                if resp.rect.bottom() > bottom {
                    break;
                }
            }
        });

        resp
    }
}
