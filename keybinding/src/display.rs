use derive_new::new;
use egui::{Response, Ui, Widget};

use crate::{KeyCommand, ShortcutState};

/// A display frame of the keyboard shortcuts
#[derive(Debug, new)]
pub struct ShortcutDisplay<'a, C: KeyCommand> {
    scs: &'a ShortcutState<C>,
}

impl<'a, C> Widget for ShortcutDisplay<'a, C>
where
    C: KeyCommand,
{
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(format!("FIXME: {:#?}", self.scs))
    }
}
