use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2};
use egui_commonmark::CommonMarkCache;

use crate::card::Card;
use crate::cmwidget::CommonMarkWidget;

#[derive(Debug, new)]
pub(crate) struct CardView<'a> {
    cards: &'a Vec<Card>,
}

impl<'a> CommonMarkWidget for CardView<'a> {
    fn ui_with_cmcache(self, ui: &mut Ui, cmcache: &mut CommonMarkCache) -> Response {
        let mut resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for card in self.cards {
                resp |= card.ui_with_cmcache(ui, cmcache);

                if resp.rect.bottom() > bottom {
                    break;
                }
            }
        });

        resp
    }
}
