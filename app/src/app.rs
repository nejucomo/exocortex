use datetime::convenience::Today as _;
use datetime::{ISO as _, LocalDate, LocalDateTime};
use eframe::egui::{
    CentralPanel, Context, Key, Response, RichText, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_page::error::PageError;
use exocortex_page::{Page, PagePath};

use crate::commandkey::CommandKey;
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

        if let Some(cmdkey) = CommandKey::get(ui) {
            use CommandKey::*;

            match cmdkey {
                Viewport(vpcmd) => ui.ctx().send_viewport_cmd(vpcmd),
                OpenNewJournal => {
                    let now = LocalDateTime::now();

                    self.open_page(
                        PagePath::from_static("journal").join(now.date().iso().to_string()),
                    )
                    .unwrap();
                }
            }
        }

        resp
    }
}

impl App {
    fn open_page(&mut self, path: PagePath) -> Result<(), PageError> {
        let page = Page::open_path(&path)?;

        self.path = path;
        self.page = page;

        Ok(())
    }
}
