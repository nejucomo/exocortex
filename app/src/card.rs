use std::collections::BTreeMap;

use eframe::egui::{Frame, Response, Ui, Widget};
use exocortex_db::messages::LogScanItems;
use exocortex_db::{CardId, Timestamp};

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

impl Widget for &Card {
    fn ui(self, ui: &mut Ui) -> Response {
        let visuals = &ui.style().visuals;

        Frame::NONE
            .stroke({
                let mut stroke = visuals.widgets.active.bg_stroke;
                stroke.color = stroke.color.gamma_multiply(0.5);
                stroke
            })
            .fill(visuals.panel_fill)
            .corner_radius(visuals.widgets.active.corner_radius * 2.0)
            .show(ui, |ui: &mut Ui| ui.label(format!("{:?}", self.id)))
            .response
    }
}
