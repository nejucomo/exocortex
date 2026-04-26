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

        if Some(&thop.ctime) != prevtime.as_ref() {
            ui.label(thop.ctime.to_string());
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
