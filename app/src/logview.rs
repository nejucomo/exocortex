use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_damo::{Card as _, Provider};

#[derive(Debug, new)]
pub(crate) struct LogView<'a, P>
where
    P: Provider,
{
    damo: &'a P,
    cmcache: &'a mut CommonMarkCache,
}

impl<'a, P> Widget for LogView<'a, P>
where
    P: Provider,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let w = ui.available_width();
        let desired = Vec2::new(w, 0.0);
        let layout = Layout::top_down(Align::Min);

        let inner = ui.allocate_ui_with_layout(desired, layout, |ui| {
            ui.set_width(w);

            let mut resp = ui.allocate_response(Vec2::ZERO, Sense::hover());

            let mut optid = self.damo.card_prev(None).unwrap();
            while let Some(id) = optid {
                let card = self.damo.open_card_ref(id).unwrap();
                let synopsis = card.get_synopsis().unwrap();

                resp |= CommonMarkViewer::new()
                    .show(ui, self.cmcache, synopsis)
                    .response;

                optid = self.damo.card_prev(Some(id)).unwrap();
            }
            resp
        });

        inner.response | inner.inner
    }
}
