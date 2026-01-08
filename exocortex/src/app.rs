use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[derive(Default)]
pub(crate) struct App {
    cmcache: CommonMarkCache,
    text: String,
    editmode: bool,
}

impl App {
    pub(crate) fn run() -> eframe::Result<()> {
        eframe::run_native(
            env!("CARGO_PKG_NAME"),
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_fullscreen(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|_cc| Ok(Box::new(Self::default()))),
        )
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(env!("CARGO_PKG_NAME"));
            if self.editmode {
                let resp = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text)
                        .lock_focus(true)
                        .id_source("text-edit-auto-focus"),
                );

                if !resp.has_focus() {
                    resp.request_focus();
                    ui.ctx().memory_mut(|mem| {
                        mem.set_focus_lock_filter(
                            resp.id,
                            egui::EventFilter {
                                tab: true,
                                horizontal_arrows: true,
                                vertical_arrows: true,
                                escape: true,
                            },
                        );
                    });
                }

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.editmode = false;
                }
            } else {
                egui::Frame::NONE
                    .stroke(egui::Stroke::new(1.0, egui::Color32::RED))
                    .show(ui, |ui| {
                        CommonMarkViewer::new().show_mut(ui, &mut self.cmcache, &mut self.text);
                    });

                if ui.input(|i| i.key_pressed(egui::Key::I)) {
                    self.editmode = true;
                }
            }
        });
    }
}
