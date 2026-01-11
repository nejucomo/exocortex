use eframe::egui::{
    CentralPanel, Context, Key, Response, RichText, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_page::{Page, PagePath};

use crate::pagewidget::PageWidget as _;

pub(crate) struct App {
    cmcache: CommonMarkCache,
    path: PagePath,
    page: Page,
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

impl Default for App {
    fn default() -> Self {
        let cmcache = CommonMarkCache::default();
        let path = PagePath::from_static("help > welcome");
        let page = Page::open_path(&path).unwrap();
        Self {
            cmcache,
            path,
            page,
        }
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

        let resp = self.page.show_page(ui, &mut self.cmcache);

        if ui.input(|i| i.key_pressed(Key::Escape) && i.modifiers.command) {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        // let resp = ui.add_sized(ui.available_size(), &mut self.textframe);

        // ui.input(|i| {
        //     if self.textframe.editmode {
        //         if i.key_pressed(Key::Escape) {
        //             self.textframe.editmode = false;
        //         }
        //     } else if i.key_pressed(Key::I) {
        //         self.textframe.editmode = true;
        //     }
        // });

        resp
    }
}
