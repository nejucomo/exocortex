use std::sync::Arc;

use derive_new::new;
use eframe::egui::mutex::Mutex;
use eframe::egui::{
    CentralPanel, Context, Event, Response, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::CommonMarkCache;
use exocortex_db::DatabaseThreadService;
use exocortex_db::messages::{DbReply, LogScan};
use exocortex_keybinding::ShortcutState;
use exocortex_widgets::squeeze_frame::UiSqueezeExt as _;
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{Orientation, UiExt, many};

use crate::command::Command;
use crate::thop::{Thop, aggregate_thop_modifications};

#[derive(new)]
pub(crate) struct App {
    db: DatabaseThreadService,

    #[new(default)]
    kbshortcuts: ShortcutState<Command>,

    /// BUG: This is locked by every common mark widget per frame!
    #[new(default)]
    cmcache: Arc<Mutex<CommonMarkCache>>,

    #[new(default)]
    thops: Vec<Thop>,
}

impl App {
    pub(crate) fn run(mut db: DatabaseThreadService) -> eframe::Result<()> {
        db.post_request(LogScan).unwrap();

        run_native(
            env!("CARGO_PKG_NAME"),
            NativeOptions {
                viewport: ViewportBuilder::default().with_maximized(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|cc| Ok(Box::new(Self::init(cc, db)))),
        )
    }

    fn init(cc: &eframe::CreationContext<'_>, db: DatabaseThreadService) -> Self {
        log::trace!("{:#?}", cc.egui_ctx.style());
        Self::new(db)
    }

    fn handle_db_reply(&mut self, reply: DbReply) {
        use DbReply::*;
        use exocortex_db::messages::Queried::LogScanned;

        match reply {
            Queried(LogScanned(items)) => {
                self.thops = aggregate_thop_modifications(&items).collect();
            }
            Modified(thop) => log::debug!("modified: {thop:?}"),
            other => panic!("unexpected db reply: {other:?}"),
        }
    }

    fn handle_ui_events(&mut self, ui: &mut Ui) {
        // TODO: Is there a better way to do this besides clone or hazardous recursive locking?
        for event in ui.input(|input| input.events.clone()) {
            self.handle_ui_event(ui, event);
        }
    }

    fn handle_ui_event(&mut self, ui: &mut Ui, event: Event) {
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
                        log::trace!("{chord:#?}");
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
            CreateNewThop => {
                todo!("FIXME")
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(reply) = self.db.poll_reply().unwrap() {
            self.handle_db_reply(reply);
        }

        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        use Orientation::Vertical;

        let resp = ui
            .within_widgets(|ui| {
                ui.scroll_area(Vertical, |ui| {
                    ui.add(many(self.thops.iter_mut()).with(&self.cmcache))
                })
            })
            .response;

        self.handle_ui_events(ui);

        resp
    }
}
