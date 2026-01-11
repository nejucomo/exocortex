use datetime::convenience::Today as _;
use datetime::{ISO as _, LocalDate, LocalDateTime};
use eframe::egui::{
    CentralPanel, Context, Key, Response, RichText, Ui, ViewportBuilder, ViewportCommand, Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_page::error::PageError;
use exocortex_page::{Page, PageDb, PagePath};

use crate::commandkey::CommandKey;
use crate::pagewidget::PageWidget as _;

#[derive(Debug, Default)]
pub(crate) struct App {
    cmcache: CommonMarkCache,
    pagedb: PageDb,
    path: PagePath,
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

        let resp = self.show_page(ui);

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
    fn show_page(&mut self, ui: &mut Ui) -> Response {
        Ok(())
    }
}
