use std::sync::Arc;

use derive_new::new;
use eframe::egui::mutex::Mutex;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_lid::WithId;
use exocortex_thop::Thop;
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{CardMode, card};

#[derive(Debug, new)]
pub(crate) struct ThopCard {
    #[new(default)]
    mode: CardMode,
    thop: WithId<Thop>,
}

impl WidgetWith<&Arc<Mutex<CommonMarkCache>>> for &mut ThopCard {
    fn ui_with(self, ui: &mut Ui, cmcache: &Arc<Mutex<CommonMarkCache>>) -> Response {
        let mode = &mut self.mode;
        let thop = &self.thop;
        ui.add(
            card()
                .mode(mode)
                .metadata(|ui: &mut Ui| {
                    let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());
                    ui.columns_const(|[left, mid, right]| {
                        r |= left
                            .with_layout(Layout::left_to_right(Align::Min), |ui| {
                                ui.label(format!("Created: {}", thop.ctime))
                            })
                            .response;

                        r |= mid
                            .vertical_centered_justified(|ui| ui.label(thop.id.to_string()))
                            .response;

                        r |= right
                            .with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.label(format!("Modified: {}", thop.mtime))
                            })
                            .response;

                        r
                    })
                })
                .summary(|ui: &mut Ui| {
                    CommonMarkViewer::new()
                        .show(
                            ui,
                            &mut cmcache.lock(),
                            thop.synopsis.lines().next().unwrap(),
                        )
                        .response
                })
                .content(|ui: &mut Ui| {
                    CommonMarkViewer::new()
                        .show(ui, &mut cmcache.lock(), &thop.synopsis)
                        .response
                })
                .build()
                .unwrap(),
        )
    }
}
