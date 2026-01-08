use derive_new::new;
use eframe::egui::{EventFilter, Response, TextEdit, Ui, Widget};

#[derive(new)]
pub(crate) struct Editor<'a> {
    textmut: &'a mut String,
}

impl<'a> Widget for Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let resp = ui.add_sized(
            ui.available_size(),
            TextEdit::multiline(self.textmut)
                .lock_focus(true)
                .id_source("text-edit-auto-focus"),
        );

        if !resp.has_focus() {
            resp.request_focus();
            ui.ctx().memory_mut(|mem| {
                mem.set_focus_lock_filter(
                    resp.id,
                    EventFilter {
                        tab: true,
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        escape: true,
                    },
                );
            });
        }

        resp
    }
}
