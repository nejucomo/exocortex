use eframe::egui::{
    CentralPanel, Context, Event, Response, RichText, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::CommonMarkCache;
use exocortex_keybinding::ShortcutState;
use exocortex_page::error::NonexistentPage;
use exocortex_page::{Page, PageDb, PagePath};
use exocortex_squeeze_frame::UiExt as _;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::command::Command;
use crate::modaleditor::ModalEditor;
use crate::viewer::Viewer;

#[derive(Debug, Default)]
pub(crate) struct App {
    kbshortcuts: ShortcutState<Command>,
    cmcache: CommonMarkCache,
    pagedb: PageDb,
    path: PagePath,
    editmode: bool,
}

impl App {
    pub(crate) fn run() -> eframe::Result<()> {
        run_native(
            env!("CARGO_PKG_NAME"),
            NativeOptions {
                viewport: ViewportBuilder::default().with_maximized(true),
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
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(self.path.as_str()).italics());
        });

        let resp = ui.within_squeeze_frame(|ui| self.show_page(ui)).response;

        self.handle_events(ui);

        resp
    }
}

impl App {
    fn show_page(&mut self, ui: &mut Ui) -> Response {
        use Page::*;

        // TODO: Don't clone every frame!
        match self.pagedb.access(self.path.clone()) {
            Ok(ReadOnly(text)) => Viewer::new(&mut self.cmcache, text).ui(ui),
            Ok(ReadWrite(text)) => {
                ModalEditor::new(&mut self.cmcache, text, &mut self.editmode).ui(ui)
            }
            Err(NonexistentPage) => todo!(),
        }
    }

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
            OpenNewJournal => {
                let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let nowstr = now.format(&Rfc3339).unwrap();
                self.path = PagePath::from_static("journal").join(nowstr);
            }
        }
    }
}
