use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::egui::mutex::Mutex;
use eframe::egui::{Align, Layout, Response, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_db::messages::LogScanItems;
use exocortex_db::{BlurbId, Timestamp};
use exocortex_widgets::card;
use exocortex_widgets::with::WidgetWith;

#[derive(Debug)]
pub(crate) struct Blurb {
    id: BlurbId,
    ctime: Timestamp,
    mtime: Timestamp,
    synopsis: String,
}

pub(crate) fn aggregate_blurb_modifications(
    modifications: &LogScanItems,
) -> impl Iterator<Item = Blurb> {
    use exocortex_db::messages::BlurbModifyG::*;

    let mut bt = BTreeMap::default();

    for (_, blurbmod) in modifications {
        let mtime = blurbmod.time;

        match &blurbmod.val {
            Create(id) => {
                let id = *id;
                assert!(
                    bt.insert(
                        id,
                        Blurb {
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
                let agg = bt.get_mut(&css.blurb).unwrap();
                agg.mtime = mtime;
                agg.synopsis = css.synopsis.clone();
            }
        }
    }

    bt.into_values()
}

impl WidgetWith<&Arc<Mutex<CommonMarkCache>>> for &Blurb {
    fn ui_with(self, ui: &mut Ui, cmcache: &Arc<Mutex<CommonMarkCache>>) -> Response {
        let resp = ui.add(card(
            |ui| {
                ui.columns_const(|[left, mid, right]| {
                    left.with_layout(Layout::left_to_right(Align::Min), |ui| {
                        ui.label(format!("Created: {}", self.ctime))
                    });

                    mid.vertical_centered_justified(|ui| ui.label(self.id.to_string()));

                    right.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.label(format!("Modified: {}", self.mtime))
                    });
                });
            },
            |ui| {
                CommonMarkViewer::new().show(
                    ui,
                    &mut cmcache.lock(),
                    self.synopsis.lines().next().unwrap(),
                );
            },
        ));

        if resp.clicked() {
            todo!("handle blurb click")
        }
        resp
    }
}
