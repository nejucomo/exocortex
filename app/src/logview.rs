use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::CommonMarkCache;

#[derive(Debug, new)]
pub(crate) struct LogView<'a> {
    #[allow(dead_code)]
    cmcache: &'a mut CommonMarkCache,
}

impl<'a> Widget for LogView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let w = ui.available_width();
        let desired = Vec2::new(w, 0.0);
        let layout = Layout::top_down(Align::Min);

        let inner = ui.allocate_ui_with_layout(desired, layout, |ui| {
            ui.set_width(w);

            let resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

            /*
            let mut optid = self.db.card_prev(None).unwrap();
            while let Some(id) = optid {
                let synopsis = self.db.card_get_synopsis(id).unwrap();

                resp |= CommonMarkViewer::new()
                    .show(ui, self.cmcache, synopsis)
                    .response;

                optid = self.db.card_prev(Some(id)).unwrap();
            }
            */
            #[allow(clippy::let_and_return)]
            resp
        });

        inner.response | inner.inner
    }
}
