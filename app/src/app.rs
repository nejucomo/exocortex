use derive_new::new;
use eframe::egui::{
    CentralPanel, Context, Event, Response, RichText, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::CommonMarkCache;
use exocortex_keybinding::ShortcutState;
use exocortex_redb::ExoDb;

use crate::command::Command;
use crate::logview::LogView;

#[derive(Debug, new)]
pub(crate) struct App {
    #[allow(dead_code)]
    db: ExoDb,

    #[new(default)]
    kbshortcuts: ShortcutState<Command>,
    #[new(default)]
    cmcache: CommonMarkCache,
}

impl App {
    pub(crate) fn run(db: ExoDb) -> eframe::Result<()> {
        run_native(
            env!("CARGO_PKG_NAME"),
            NativeOptions {
                viewport: ViewportBuilder::default().with_maximized(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|_cc| Ok(Box::new(Self::new(db)))),
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
        let mut resp = ui
            .vertical_centered(|ui| {
                ui.label(RichText::new("exocortex").italics());
            })
            .response;

        resp |= ui.add(LogView::new(&mut self.cmcache));

        self.handle_events(ui);

        resp
    }
}

impl App {
    fn handle_events(&mut self, ui: &mut Ui) {
        // TODO: Is there a better way to do this besides clone or hazardous recursive locking?
        for event in ui.input(|input| input.events.clone()) {
            self.handle_event(ui, event);
        }
    }

    fn handle_event(&mut self, ui: &mut Ui, event: Event) {
        use eframe::egui::Event::Key;
        use exocortex_keybinding::HandleKey::*;

        match event {
            Key {
                key,
                modifiers,
                pressed: true,
                ..
            } => match self.kbshortcuts.handle_key((key, modifiers)) {
                Ok(hk) => match hk {
                    Pending => {
                        // ok...
                    }
                    Command(cmd) => self.handle_command(ui, cmd),
                    Unhandled(chord) => {
                        dbg!("{chord:#?}");
                        let _ = chord;
                    }
                },
                Err(_) => todo!(),
            },

            _ => {
                // Ignored
            }
        }
    }

    fn handle_command(&mut self, ui: &mut Ui, cmd: Command) {
        use Command::*;
        use ViewportCommand::Fullscreen;

        match cmd {
            Viewport(vpcmd) => ui.ctx().send_viewport_cmd(vpcmd),
            ViewportToggleFullscreen => {
                let fs = ui.input(|i| i.viewport().fullscreen.unwrap_or_default());
                ui.ctx().send_viewport_cmd(Fullscreen(!fs));
            }
            CreateNewCard => {
                todo!("FIXME")
            }
        }
    }
}
