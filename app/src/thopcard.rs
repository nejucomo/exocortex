use derive_more::Deref;
use derive_new::new;
use eframe::egui::{Response, RichText, TextEdit, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use exocortex_lid::WithId;
use exocortex_thop::Thop;
use exocortex_timestamp::Timestamp;
use exocortex_widgets::with::WidgetWith;
use exocortex_widgets::{CardMode, card};

#[derive(Debug, new, Deref)]
pub(crate) struct ThopCard {
    #[new(default)]
    pub mode: CardMode,
    #[deref]
    thop: WithId<Thop>,
}

impl WidgetWith<(Option<Timestamp>, &mut CommonMarkCache)> for &mut ThopCard {
    fn ui_with(
        self,
        ui: &mut Ui,
        (prevtime, cmcache): (Option<Timestamp>, &mut CommonMarkCache),
    ) -> Response {
        let modemut = &mut self.mode;
        let thop = &mut self.thop;
        let ctime = &thop.ctime;

        let (show_date, show_hm) = if let Some(pt) = prevtime {
            use jiff::{Unit::Minute, ZonedDifference};

            let delta_m = pt
                .until(
                    ZonedDifference::from(ctime.as_zoned())
                        .smallest(Minute)
                        .largest(Minute),
                )
                .unwrap()
                .get_minutes();

            let show_date = pt.date() != ctime.date();
            let show_hm = delta_m > 0;
            (show_date, show_hm)
        } else {
            (true, true)
        };

        if show_date {
            ui.label(RichText::new(ctime.date().to_string()).small().strong());
        }
        if show_date || show_hm {
            ui.label(RichText::new(ctime.strftime("%H:%M").to_string()).small());
        }

        ui.add(
            card()
                .mode(modemut)
                .content(|ui: &mut Ui, mode: CardMode| {
                    use CardMode::*;

                    let mut cmviewer =
                        |text| CommonMarkViewer::new().show(ui, cmcache, text).response;

                    match mode {
                        Streamlined => cmviewer(thop.synopsis.lines().next().unwrap()),
                        Expanded => cmviewer(thop.synopsis.as_str()),
                        Editing => {
                            let thopid = thop.id;
                            let resp = ui.add(
                                TextEdit::singleline(&mut thop.synopsis)
                                    .id_salt(("ThopCard TextEdit", thopid))
                                    .desired_width(ui.available_width()),
                            );

                            // What a yuck API: `resp.lost_focus` implies the user pressed `<enter>` within the text edit box...
                            if resp.lost_focus() {
                                todo!("lost focus! {:?}", &thop.synopsis);
                            } else if !resp.has_focus() {
                                resp.request_focus();
                            }

                            resp
                        }
                    }
                })
                .build()
                .unwrap(),
        )
    }
}
