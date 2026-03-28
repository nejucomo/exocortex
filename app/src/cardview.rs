use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::aggregate::CardAgg;

#[derive(Debug, new)]
pub(crate) struct CardView<'a> {
    cmcache: &'a mut CommonMarkCache,
    cards: &'a Vec<CardAgg>,
}

impl<'a> Widget for CardView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut any = ui.allocate_response(Vec2::ZERO, Sense::hover());
        let mut overflowed = false;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for card in self.cards {
                let r = CommonMarkViewer::new()
                    .show(ui, self.cmcache, &format!("```{card:?}```"))
                    .response;

                any |= r.clone();

                if r.rect.bottom() > bottom {
                    overflowed = true;
                    break;
                }
            }
        });

        any
    }
}
