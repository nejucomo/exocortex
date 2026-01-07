use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

const MD: &str = r"
# Hello world

* A list
* [ ] Checkbox
";

#[derive(Default)]
pub(crate) struct App {}

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
        let mut cache = CommonMarkCache::default();

        egui::CentralPanel::default().show(ctx, |ui| {
            CommonMarkViewer::new().show(ui, &mut cache, MD);
        });
    }
}
