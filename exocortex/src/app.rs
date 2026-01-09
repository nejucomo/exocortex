use eframe::egui::{
    CentralPanel, Context, Key, Response, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};

use crate::textframe::TextFrame;

#[derive(Default)]
pub(crate) struct App {
    textframe: TextFrame,
}

impl App {
    pub(crate) fn run() -> eframe::Result<()> {
        run_native(
            env!("CARGO_PKG_NAME"),
            NativeOptions {
                viewport: ViewportBuilder::default().with_fullscreen(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|_cc| Ok(Box::new(Self::default()))),
        )
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(env!("CARGO_PKG_NAME"));
        let resp = ui.add_sized(ui.available_size(), &mut self.textframe);

        if ui.input(|i| i.key_pressed(Key::Escape) && i.modifiers.command) {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        ui.input(|i| {
            if self.textframe.editmode {
                if i.key_pressed(Key::Escape) {
                    self.textframe.editmode = false;
                }
            } else if i.key_pressed(Key::I) {
                self.textframe.editmode = true;
            }
        });

        resp
    }
}
