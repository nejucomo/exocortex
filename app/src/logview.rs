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
}

impl<'a> Widget for LogView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let w = ui.available_width();
        let desired = Vec2::new(w, 0.0);
        let layout = Layout::top_down(Align::Min);

        let inner = ui.allocate_ui_with_layout(desired, layout, |ui| {
            ui.set_width(w);

            let mut resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

            for synopsis in self.cards {
                if ui.available_height() > 0.0 {
                    resp |= CommonMarkViewer::new()
                        .show(ui, self.cmcache, synopsis)
                        .response;
                } else {
                    break;
                }
            }

            self.dbman
                .post_scan_request_if_none_outstanding(if ui.available_height() > 0.0 {
                    CardScan::Next
                } else {
                    CardScan::Stop
                });

            resp
        });

        inner.response | inner.inner
    }
}
