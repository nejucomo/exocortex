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
            // TODO: Get rid of all this boxing if possible:
            Box::new(|cc| Ok(Box::new(App::new(cc)))),
        )
    }

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.editmode {
                let resp = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text).lock_focus(true),
                );
                ui.memory_mut(|mem| mem.request_focus(resp.id));

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.editmode = false;
                }
            } else {
                CommonMarkViewer::new().show_mut(ui, &mut self.cmcache, &mut self.text);
                if ui.input(|i| i.key_pressed(egui::Key::I)) {
                    self.editmode = true;
                }
            }
        });
    }
}
