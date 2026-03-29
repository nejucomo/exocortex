use std::collections::BTreeMap;

use eframe::egui::{Align, Frame, Layout, Response, Ui};
use eframe::epaint::MarginF32;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_db::messages::LogScanItems;
use exocortex_db::{CardId, Timestamp};

use crate::cmwidget::CommonMarkWidget;

const CORNER_RADIUS: f32 = 6.0;
const INNER_MARGIN: MarginF32 = MarginF32::symmetric(6.0, 2.0);
const STROKE_GAMMA: f32 = 0.5;
const STROKE_WIDTH: f32 = 1.0;

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
        show_card_frame(
            ui,
            Frame::NONE
                .stroke({
                    let mut stroke = ui.style().visuals.widgets.active.bg_stroke;
                    stroke.color = stroke.color.gamma_multiply(STROKE_GAMMA);
                    stroke.width = STROKE_WIDTH;
                    stroke
                })
                .corner_radius(CORNER_RADIUS)
                .inner_margin(INNER_MARGIN),
            |ui| {
                ui.with_layout(Layout::top_down(Align::Max), |ui| {
                    ui.columns_const(|[left, mid, right]| {
                        use eframe::egui::{RichText, TextStyle::Small};

                        left.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            ui.label(
                                RichText::new(format!("Created: {}", self.ctime)).text_style(Small),
                            )
                        });

                        mid.vertical_centered_justified(|ui| {
                            ui.label(RichText::new(format!("{}", self.id)).text_style(Small))
                        });

                        right.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ui.label(
                                RichText::new(format!("Modified: {}", self.mtime))
                                    .text_style(Small),
                            )
                        });
                    });

                    Frame::NONE.show(ui, |ui: &mut Ui| {
                        CommonMarkViewer::new()
                            .show(ui, cmcache, self.synopsis.lines().next().unwrap())
                            .response
                    })
                });
            },
        )
    }
}

fn show_card_frame<F>(ui: &mut Ui, f: Frame, mkui: F) -> Response
where
    F: FnOnce(&mut Ui),
{
    let mut prep = f.begin(ui);

    mkui(&mut prep.content_ui);

    let resp = prep.allocate_space(ui);
    let visuals = ui.style().visuals.widgets.style(&resp);

    log_visuals_if_necessary(visuals);

    prep.frame.fill = visuals.bg_fill;
    prep.frame.stroke = visuals.bg_stroke;

    /*
    pub weak_bg_fill: Color32,
    pub bg_stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub fg_stroke: Stroke,
    pub expansion: f32,
    */

    prep.end(ui)
}

fn log_visuals_if_necessary(v: &eframe::egui::style::WidgetVisuals) {
    use std::sync::{Arc, LazyLock, Mutex};

    static PREV_VISUALS: LazyLock<Arc<Mutex<Option<eframe::egui::style::WidgetVisuals>>>> =
        LazyLock::new(|| Arc::new(Mutex::new(None)));

    let mut optprev = PREV_VISUALS.lock().unwrap();

    let stored = optprev.get_or_insert(*v);
    if stored != v {
        log::trace!("applying card visuals: {:#?}", (v.bg_fill, v.bg_stroke));
        *stored = *v;
    }
}
