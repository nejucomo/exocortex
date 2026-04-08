use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::egui::mutex::Mutex;
use eframe::egui::{Align, Layout, Response, Sense, Ui, Vec2};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_db::messages::LogScanItems;
use exocortex_db::{ThopId, Timestamp};
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{CardMode, card};

#[derive(Debug)]
pub(crate) struct ThopAggregate {
    mode: CardMode,
    id: ThopId,
    ctime: Timestamp,
    mtime: Timestamp,
    synopsis: String,
}

pub(crate) fn aggregate_thop_modifications(
    modifications: &LogScanItems,
) -> impl Iterator<Item = ThopAggregate> {
    use exocortex_db::messages::ThopModifyG::*;

    let mut bt = BTreeMap::default();

    for (_, thopmod) in modifications {
        let mtime = thopmod.time;

        match &thopmod.val {
            Create(id) => {
                let id = *id;
                assert!(
                    bt.insert(
                        id,
                        ThopAggregate {
                            mode: CardMode::Streamlined,
                            id,
                            ctime: mtime,
                            mtime,
                            synopsis: "".to_string()
                        }
                    )
                    .is_none()
                );
            }
            SetSynopsis(css) => {
                let agg = bt.get_mut(&css.thop).unwrap();
                agg.mtime = mtime;
                agg.synopsis = css.synopsis.clone();
            }
        }
    }

    bt.into_values()
}

impl WidgetWith<&Arc<Mutex<CommonMarkCache>>> for &mut ThopAggregate {
    fn ui_with(self, ui: &mut Ui, cmcache: &Arc<Mutex<CommonMarkCache>>) -> Response {
        ui.add(
            card()
                .mode(&mut self.mode)
                .metadata(|ui: &mut Ui| {
                    let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());
                    ui.columns_const(|[left, mid, right]| {
                        r |= left
                            .with_layout(Layout::left_to_right(Align::Min), |ui| {
                                ui.label(format!("Created: {}", self.ctime))
                            })
                            .response;

                        r |= mid
                            .vertical_centered_justified(|ui| ui.label(self.id.to_string()))
                            .response;

                        r |= right
                            .with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.label(format!("Modified: {}", self.mtime))
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
                            self.synopsis.lines().next().unwrap(),
                        )
                        .response
                })
                .content(|ui: &mut Ui| {
                    CommonMarkViewer::new()
                        .show(ui, &mut cmcache.lock(), &self.synopsis)
                        .response
                })
                .build()
                .unwrap(),
        )
    }
}
