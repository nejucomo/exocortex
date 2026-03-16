use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_redb::messages::CardScan;

use crate::dbman::DbManager;

#[derive(Debug, new)]
pub(crate) struct LogView<'a> {
    dbman: &'a mut DbManager,
    cmcache: &'a mut CommonMarkCache,
    cards: &'a [String],
    scan_complete: bool,
}

impl<'a> Widget for LogView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut any = ui.allocate_response(Vec2::ZERO, Sense::hover());
        let mut overflowed = false;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for synopsis in self.cards {
                let r = CommonMarkViewer::new()
                    .show(ui, self.cmcache, synopsis)
                    .response;

                any |= r.clone();

                if r.rect.bottom() > bottom {
                    overflowed = true;
                    break;
                }
            }

            any |= ui.label(if self.scan_complete {
                "<scan complete>"
            } else {
                "<scan incomplete>"
            });
        });

        self.dbman
            .post_scan_request_if_none_outstanding(if overflowed {
                CardScan::Stop
            } else {
                CardScan::Next
            });

        any
    }
}
