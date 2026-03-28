use std::collections::BTreeMap;

use eframe::egui::{Frame, Response, RichText, TextStyle, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_db::messages::LogScanItems;
use exocortex_db::{CardId, Timestamp};

use crate::cmwidget::CommonMarkWidget;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Card {
    id: CardId,
    ctime: Timestamp,
    mtime: Timestamp,
    synopsis: String,
}

pub(crate) fn aggregate_card_modifications(
    modifications: &LogScanItems,
) -> impl Iterator<Item = Card> {
    use exocortex_db::messages::CardModifyG::*;

    let mut bt = BTreeMap::default();

    for (_, cardmod) in modifications {
        let mtime = cardmod.time;

        match &cardmod.val {
            Create(id) => {
                let id = *id;
                assert!(
                    bt.insert(
                        id,
                        Card {
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
                let agg = bt.get_mut(&css.card).unwrap();
                agg.mtime = mtime;
                agg.synopsis = css.synopsis.clone();
            }
        }
    }

    bt.into_values()
}

impl CommonMarkWidget for &Card {
    fn ui_with_cmcache(self, ui: &mut Ui, cmcache: &mut CommonMarkCache) -> Response {
        let visuals = &ui.style().visuals;

        Frame::NONE
            .stroke({
                let mut stroke = visuals.widgets.active.bg_stroke;
                stroke.color = stroke.color.gamma_multiply(0.5);
                stroke
            })
            .fill(visuals.panel_fill)
            .corner_radius(visuals.widgets.active.corner_radius * 2.0)
            .show(ui, |ui: &mut Ui| {
                let mut r = ui.label(
                    RichText::new(format!(
                        "[{}] Created: {} Modified: {}",
                        self.id, self.ctime, self.mtime
                    ))
                    .text_style(TextStyle::Small),
                );

                r |= CommonMarkViewer::new()
                    .show(ui, cmcache, &self.synopsis)
                    .response;
            })
            .response
    }
}
