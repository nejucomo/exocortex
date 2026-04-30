use derive_new::new;
use eframe::egui::{
    CentralPanel, Context, Event, Response, Sense, Ui, Vec2, ViewportBuilder, ViewportCommand,
    Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::CommonMarkCache;
use exocortex_keybinding::ShortcutState;
use exocortex_lid::WithId;
use exocortex_lid::{Id, IdMap};
use exocortex_memory::Provider;
use exocortex_memory::modifications::{ThopCreate, ThopModified};
use exocortex_memory::queries::{Scan, ScanNext, ScanQueried, ScanReleased};
use exocortex_memory::{Reply, ReplyInfo};
use exocortex_thop::Thop;
use exocortex_widgets::squeeze_frame::UiSqueezeExt as _;
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{Orientation, UiExt};

use crate::command::Command;
use crate::thopcard::ThopCard;

struct ScanInProgress {
    scan_id: Id<Scan>,
    collected: Vec<WithId<ThopModified>>,
}

#[derive(new)]
pub(crate) struct App<P: Provider> {
    db: P,

    #[new(default)]
    kbshortcuts: ShortcutState<Command>,

    /// BUG: This is locked by every common mark widget per frame!
    #[new(default)]
    cmcache: CommonMarkCache,

    #[new(default)]
    thop_editing: Option<Id<Thop>>,

    #[new(default)]
    thops: Vec<ThopCard>,

    #[new(default)]
    scan: Option<ScanInProgress>,
}

impl<P: Provider> App<P> {
    pub(crate) fn run(mut db: P) -> eframe::Result<()>
    where
        P: Send + 'static,
        P::Error: std::fmt::Debug,
    {
        db.post_subrequest(Scan).unwrap();

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

    fn init(cc: &eframe::CreationContext<'_>, db: P) -> Self {
        log::trace!("{:#?}", cc.egui_ctx.style());
        Self::new(db)
    }

    fn handle_db_reply(&mut self, reply: Reply) {
        match reply.reply_info {
            ReplyInfo::Queried(queried) => {
                use exocortex_memory::queries::Queried::*;
                match queried {
                    ThopCounted(_) => {
                        log::debug!("thop count reply (unexpected in app)");
                    }
                    Scanned(scan_queried) => self.handle_scan_queried(scan_queried),
                }
            }
            ReplyInfo::Modified(widtmod) => {
                use exocortex_memory::modifications::ThopMutation::*;

                match &widtmod.info {
                    Created => {
                        self.thop_editing = Some(widtmod.thop);
                        self.db.post_subrequest(Scan).unwrap();
                    }
                    SetSynopsis(syn) => {
                        // Unset editing marker:
                        let thop = self.thop_editing.take().unwrap();
                        // TODO: Should we use an { id -> state } map instead?
                        for tc in self.thops.iter_mut() {
                            if tc.id == thop {
                                assert_eq!(syn, &tc.synopsis);
                                tc.mode = exocortex_widgets::CardMode::Expanded;
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_scan_queried(&mut self, sq: ScanQueried) {
        match sq {
            ScanQueried::Started(scan_id) => {
                self.scan = Some(ScanInProgress {
                    scan_id,
                    collected: Vec::new(),
                });
                self.db.post_subrequest(ScanNext(scan_id)).unwrap();
            }
            ScanQueried::Advanced(item) => {
                if let Some(scan) = self.scan.as_mut() {
                    let scan_id = scan.scan_id;
                    scan.collected.push(item);
                    self.db.post_subrequest(ScanNext(scan_id)).unwrap();
                }
            }
            ScanQueried::Released(ScanReleased) => {
                if let Some(scan) = self.scan.take() {
                    // self.thops = aggregate_thop_modifications(&scan.collected).collect();
                    let tmap: IdMap<Thop> = scan
                        .collected
                        .iter()
                        .try_fold(IdMap::default(), |mut tmap, wid| {
                            wid.value.modify_thop_map(&mut tmap).map(|()| tmap)
                        })
                        .unwrap();

                    self.thops = tmap.into_iter().map(ThopCard::new).collect();
                }
            }
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
                self.db.post_subrequest(ThopCreate).unwrap();
            }
        }
    }
}

impl<P: Provider> eframe::App for App<P>
where
    P::Error: std::fmt::Debug,
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(reply) = self.db.poll_reply().unwrap() {
            self.handle_db_reply(reply);
            // egui only repaints when there is user interaction; request an
            // explicit repaint so incremental scan replies update the display.
            ctx.request_repaint();
        }

        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl<P: Provider> Widget for &mut App<P>
where
    P::Error: std::fmt::Debug,
{
    fn ui(self, ui: &mut Ui) -> Response {
        use Orientation::Vertical;

        let resp = ui
            .within_widgets(|ui| {
                ui.scroll_area(Vertical, |ui| {
                    let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());
                    let mut prevtime = None;

                    for tc in self.thops.iter_mut() {
                        let pt = prevtime.take();
                        prevtime = Some(tc.ctime.clone());

                        // A hack for editing... suspicious smell
                        use exocortex_widgets::CardMode::{Editing, Expanded};

                        if self.thop_editing.map(|id| id == tc.id).unwrap_or(false) {
                            tc.mode = Editing;
                        } else if matches!(tc.mode, Editing) {
                            tc.mode = Expanded;
                        }

                        r |= ui.add(tc.with((pt, &mut self.db, &mut self.cmcache)));
                    }

                    r
                })
            })
            .response;

        self.handle_ui_events(ui);

        resp
    }
}
