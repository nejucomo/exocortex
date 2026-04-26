use std::sync::Arc;

use derive_new::new;
use eframe::egui::mutex::Mutex;
use eframe::egui::{Response, Ui};
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
                .metadata(|ui: &mut Ui, _mode: CardMode| {
                    ui.label(format!("Created: {}", thop.ctime))
                })
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
