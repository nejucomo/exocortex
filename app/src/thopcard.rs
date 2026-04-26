use std::sync::Arc;

use derive_more::Deref;
use derive_new::new;
use eframe::egui::mutex::Mutex;
use eframe::egui::{Response, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_lid::WithId;
use exocortex_thop::Thop;
use exocortex_timestamp::Timestamp;
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{CardMode, card};

#[derive(Debug, new, Deref)]
pub(crate) struct ThopCard {
    #[new(default)]
    mode: CardMode,
    #[deref]
    thop: WithId<Thop>,
}

impl WidgetWith<(Option<Timestamp>, &Arc<Mutex<CommonMarkCache>>)> for &mut ThopCard {
    fn ui_with(
        self,
        ui: &mut Ui,
        (prevtime, cmcache): (Option<Timestamp>, &Arc<Mutex<CommonMarkCache>>),
    ) -> Response {
        let mode = &mut self.mode;
        let thop = &self.thop;
        let ctime = &thop.ctime;

        let (show_date, show_hm) = if let Some(pt) = prevtime {
            use jiff::{Unit::Minute, ZonedDifference};

            let delta_m = pt
                .until(
                    ZonedDifference::from(ctime.as_zoned())
                        .smallest(Minute)
                        .largest(Minute),
                )
                .unwrap()
                .get_minutes();

            let show_date = pt.date() != ctime.date();
            let show_hm = show_date || delta_m > 0;
            (show_date, show_hm)
        } else {
            (true, true)
        };

        if show_date {
            ui.label(ctime.date().to_string());
        }
        if show_hm {
            ui.label(ctime.time().to_string());
        }

        ui.add(
            card()
                .mode(mode)
                .content(|ui: &mut Ui, mode: CardMode| {
                    use CardMode::*;

                    CommonMarkViewer::new()
                        .show(
                            ui,
                            &mut cmcache.lock(),
                            match mode {
                                Streamlined => thop.synopsis.lines().next().unwrap(),
                                Expanded => thop.synopsis.as_str(),
                            },
                        )
                        .response
                })
                .build()
                .unwrap(),
        )
    }
}
