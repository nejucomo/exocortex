use derive_new::new;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2, Widget};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_db::DatabaseThreadService;
use exocortex_db::messages::ScannedItems;

#[derive(Debug, new)]
pub(crate) struct LogView<'a> {
    #[allow(dead_code)]
    db: &'a mut DatabaseThreadService,
    cmcache: &'a mut CommonMarkCache,
    scanned: &'a ScannedItems,
}

impl<'a> Widget for LogView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut any = ui.allocate_response(Vec2::ZERO, Sense::hover());
        let mut overflowed = false;

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.set_max_width(ui.available_width());

            let bottom = ui.clip_rect().bottom();

            for (modid, modify) in self.scanned {
                let r = CommonMarkViewer::new()
                    .show(ui, self.cmcache, &format!("```{modid:?} | {modify:?}```"))
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
