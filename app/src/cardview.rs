use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::card::Card;

#[derive(Debug, new)]
pub(crate) struct CardView<'a> {
    cmcache: &'a mut CommonMarkCache,
    cards: &'a Vec<Card>,
}

impl<'a> Widget for CardView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for card in self.cards {
                resp |= ui.add(card);

                let r = CommonMarkViewer::new()
                    .show(ui, self.cmcache, &format!("```{card:?}```"))
                    .response;

                let rb = r.rect.bottom();
                resp |= r;

                if rb > bottom {
                    break;
                }
            }
        });

        resp
    }
}
