use crate::model::{Action, Jot};
use crate::storage::Store;
use chrono::Utc;
use egui::{Align, Key, KeyboardShortcut, Modifiers, ViewportCommand};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers as HKModifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use uuid::Uuid;

/// Which top-level view is displayed in the main area.
#[derive(Debug, Clone, PartialEq)]
enum View {
    Jots,
    Log,
}

/// State for the quick-jot overlay dialog.
struct QuickJotOverlay {
    active: bool,
    text: String,
    /// Saved view to restore once the dialog closes.
    saved_view: Option<View>,
    focus_text: bool,
}

impl QuickJotOverlay {
    fn new() -> Self {
        Self {
            active: false,
            text: String::new(),
            saved_view: None,
            focus_text: false,
        }
    }

    fn activate(&mut self, current_view: &View) {
        self.active = true;
        self.text.clear();
        self.saved_view = Some(current_view.clone());
        self.focus_text = true;
    }

    /// Returns the previously saved view.
    fn deactivate(&mut self) -> Option<View> {
        self.active = false;
        self.text.clear();
        self.saved_view.take()
    }
}

/// The main eframe application struct.
pub struct App {
    store: Store,
    /// In-memory cache of actions; always consistent with the on-disk log.
    actions: Vec<Action>,
    /// Computed jots (replayed from actions).
    jots: Vec<Jot>,
    current_view: View,
    quick_jot: QuickJotOverlay,
    // Jots-view filter state
    search_text: String,
    date_from: String,
    date_to: String,
    // Status bar message
    status: String,
    // Global hotkey manager (None if registration fails)
    _hotkey_manager: Option<GlobalHotKeyManager>,
    hotkey_id: Option<u32>,
    // Inline edit state: (jot id, edit buffer)
    editing_jot: Option<(uuid::Uuid, String)>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let store = match Store::open() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Warning: could not open store: {e}");
                Store::open_at(std::env::temp_dir().join("exocortex-fallback")).unwrap()
            }
        };

        let actions = store.load_actions().unwrap_or_default();
        let jots = Store::compute_jots(&actions);

        // Attempt to register global hotkey (Ctrl+Shift+J); failures are non-fatal.
        let quick_jot_hotkey =
            HotKey::new(Some(HKModifiers::CONTROL | HKModifiers::SHIFT), Code::KeyJ);
        let (hotkey_manager, hotkey_id) = match GlobalHotKeyManager::new() {
            Ok(manager) => {
                let id = quick_jot_hotkey.id();
                match manager.register(quick_jot_hotkey) {
                    Ok(()) => (Some(manager), Some(id)),
                    Err(e) => {
                        eprintln!("Could not register global hotkey: {e}");
                        (Some(manager), None)
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not create hotkey manager: {e}");
                (None, None)
            }
        };

        Self {
            store,
            actions,
            jots,
            current_view: View::Jots,
            quick_jot: QuickJotOverlay::new(),
            search_text: String::new(),
            date_from: String::new(),
            date_to: String::new(),
            status: String::new(),
            _hotkey_manager: hotkey_manager,
            hotkey_id,
            editing_jot: None,
        }
    }

    /// Persist an action and update the in-memory state.
    fn commit_action(&mut self, action: Action) {
        if let Err(e) = self.store.append(&action) {
            self.status = format!("Error saving: {e}");
            return;
        }
        self.actions.push(action);
        self.jots = Store::compute_jots(&self.actions);
    }

    fn add_jot(&mut self, text: String) {
        if text.trim().is_empty() {
            return;
        }
        let action = Action::AddJot {
            timestamp: Utc::now(),
            id: Uuid::new_v4(),
            text: text.trim().to_string(),
        };
        self.commit_action(action);
        self.status = "Jot added.".into();
    }

    fn commit_edit(&mut self, id: uuid::Uuid, new_text: String) {
        if new_text.trim().is_empty() {
            self.status = "Edit discarded (empty text).".into();
            return;
        }
        let action = Action::EditJot {
            timestamp: Utc::now(),
            id,
            new_text: new_text.trim().to_string(),
        };
        self.commit_action(action);
        self.status = "Jot updated.".into();
    }

    /// Poll for global hotkey events and activate quick-jot if triggered.
    fn check_hotkey(&mut self, ctx: &egui::Context) {
        let Some(hotkey_id) = self.hotkey_id else {
            return;
        };
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.id() == hotkey_id && event.state() == HotKeyState::Pressed {
                if self.quick_jot.active {
                    self.quick_jot.deactivate();
                } else {
                    ctx.send_viewport_cmd(ViewportCommand::Focus);
                    ctx.send_viewport_cmd(ViewportCommand::Visible(true));
                    let view = self.current_view.clone();
                    self.quick_jot.activate(&view);
                }
            }
        }
    }

    fn show_quick_jot(&mut self, ctx: &egui::Context) {
        if !self.quick_jot.active {
            return;
        }

        // Dim background.
        let screen_rect = ctx.content_rect();
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("quick_jot_overlay"),
        ));
        painter.rect_filled(screen_rect, 0.0, egui::Color32::from_black_alpha(120));

        let mut submitted = false;
        let mut cancelled = false;

        egui::Window::new("Quick Jot")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([420.0, 90.0])
            .show(ctx, |ui| {
                ui.label("Type a new jot and press Enter:");
                let response = ui.add_sized(
                    [ui.available_width(), 32.0],
                    egui::TextEdit::singleline(&mut self.quick_jot.text)
                        .hint_text("Enter jot text…")
                        .font(egui::TextStyle::Heading),
                );

                if self.quick_jot.focus_text {
                    response.request_focus();
                    self.quick_jot.focus_text = false;
                }

                if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    submitted = true;
                }
                if ui.input(|i| i.key_pressed(Key::Escape)) {
                    cancelled = true;
                }

                ui.horizontal(|ui| {
                    if ui.button("Add  [Enter]").clicked() {
                        submitted = true;
                    }
                    if ui.button("Cancel  [Esc]").clicked() {
                        cancelled = true;
                    }
                });
            });

        if submitted {
            let text = self.quick_jot.text.clone();
            self.add_jot(text);
            if let Some(view) = self.quick_jot.deactivate() {
                self.current_view = view;
            }
        } else if cancelled {
            if let Some(view) = self.quick_jot.deactivate() {
                self.current_view = view;
            }
        }
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Exocortex");
            ui.separator();

            let hotkey_hint = if self.hotkey_id.is_some() {
                " (Ctrl+Shift+J)"
            } else {
                ""
            };

            if ui
                .selectable_label(false, format!("⚡ Quick Jot{hotkey_hint}"))
                .clicked()
            {
                let view = self.current_view.clone();
                self.quick_jot.activate(&view);
            }

            ui.separator();

            if ui
                .selectable_label(self.current_view == View::Jots, "📝 Jots")
                .clicked()
            {
                self.current_view = View::Jots;
            }
            if ui
                .selectable_label(self.current_view == View::Log, "📋 Log")
                .clicked()
            {
                self.current_view = View::Log;
            }
        });
    }

    fn show_jots_view(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("🔍 Filters")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_text)
                            .hint_text("case-insensitive substring"),
                    );
                    if ui.small_button("✕").on_hover_text("Clear search").clicked() {
                        self.search_text.clear();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("From:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.date_from)
                            .hint_text("2024-01-01T00:00:00Z"),
                    );
                    ui.label("To:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.date_to)
                            .hint_text("2024-12-31T23:59:59Z"),
                    );
                    if ui
                        .small_button("✕")
                        .on_hover_text("Clear date range")
                        .clicked()
                    {
                        self.date_from.clear();
                        self.date_to.clear();
                    }
                });
            });

        ui.separator();

        let from_ts = chrono::DateTime::parse_from_rfc3339(&self.date_from)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc));
        let to_ts = chrono::DateTime::parse_from_rfc3339(&self.date_to)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc));
        let search = if self.search_text.is_empty() {
            None
        } else {
            Some(self.search_text.as_str())
        };

        let filtered: Vec<Jot> = Store::filter_jots(&self.jots, search, from_ts, to_ts)
            .into_iter()
            .cloned()
            .collect();

        ui.label(format!("{} / {} jots", filtered.len(), self.jots.len()));
        ui.separator();

        let mut pending_edit_commit: Option<(uuid::Uuid, String)> = None;
        let mut pending_edit_start: Option<(uuid::Uuid, String)> = None;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for jot in &filtered {
                    let is_editing = self
                        .editing_jot
                        .as_ref()
                        .map(|(id, _)| *id == jot.id)
                        .unwrap_or(false);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(
                                jot.created_at.format("%Y-%m-%d %H:%M").to_string(),
                            )
                            .weak()
                            .small(),
                        );

                        if is_editing {
                            let buf = &mut self.editing_jot.as_mut().unwrap().1;
                            let resp =
                                ui.add(egui::TextEdit::singleline(buf).desired_width(f32::INFINITY));
                            if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                                let (id, text) = self.editing_jot.clone().unwrap();
                                pending_edit_commit = Some((id, text));
                            }
                            if ui.input(|i| i.key_pressed(Key::Escape)) {
                                self.editing_jot = None;
                            }
                            if ui.small_button("✓").on_hover_text("Save [Enter]").clicked() {
                                let (id, text) = self.editing_jot.clone().unwrap();
                                pending_edit_commit = Some((id, text));
                            }
                        } else {
                            let label = ui.selectable_label(false, &jot.text);
                            if label.double_clicked() {
                                pending_edit_start = Some((jot.id, jot.text.clone()));
                            }
                            label.on_hover_text("Double-click to edit");
                        }
                    });
                    ui.separator();
                }
            });

        if let Some((id, text)) = pending_edit_commit {
            let original = self.jots.iter().find(|j| j.id == id).map(|j| j.text.clone());
            if original.as_deref() != Some(&text) {
                self.commit_edit(id, text);
            }
            self.editing_jot = None;
        } else if let Some((id, text)) = pending_edit_start {
            self.editing_jot = Some((id, text));
        }
    }

    fn show_log_view(&self, ui: &mut egui::Ui) {
        ui.heading("Action Log");
        ui.label(format!("{} entries", self.actions.len()));
        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for action in &self.actions {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(
                                action.timestamp().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                            )
                            .weak()
                            .small(),
                        );
                        ui.label(action.description());
                    });
                    ui.separator();
                }
                if self.actions.is_empty() {
                    ui.label("(no actions yet)");
                }
            });
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll for global hotkey events.
        self.check_hotkey(ctx);

        // In-app keyboard shortcut for quick jot.
        let open_quick_jot = ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut::new(
                Modifiers::CTRL | Modifiers::SHIFT,
                Key::J,
            ))
        });
        if open_quick_jot && !self.quick_jot.active {
            let view = self.current_view.clone();
            self.quick_jot.activate(&view);
        }

        // Keep repainting to poll hotkey events even when idle.
        if self.hotkey_id.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Top menu bar
        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quick Jot  Ctrl+Shift+J").clicked() {
                        let view = self.current_view.clone();
                        self.quick_jot.activate(&view);
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Jots").clicked() {
                        self.current_view = View::Jots;
                        ui.close();
                    }
                    if ui.button("Log").clicked() {
                        self.current_view = View::Log;
                        ui.close();
                    }
                });
            });
        });

        // Status bar
        egui::Panel::bottom("status_bar").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                if !self.status.is_empty() {
                    ui.label(egui::RichText::new(&self.status).weak());
                }
                if self.hotkey_id.is_some() {
                    ui.label(
                        egui::RichText::new("global hotkey: Ctrl+Shift+J")
                            .weak()
                            .small(),
                    );
                }
            });
        });

        // Left sidebar
        egui::Panel::left("sidebar")
            .resizable(false)
            .default_size(160.0)
            .show_inside(ui, |ui| {
                self.show_sidebar(ui);
            });

        // Main content area
        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.current_view {
                View::Jots => self.show_jots_view(ui),
                View::Log => self.show_log_view(ui),
            }
        });

        // Quick-jot overlay (rendered on top of everything)
        self.show_quick_jot(&ctx);
    }
}
