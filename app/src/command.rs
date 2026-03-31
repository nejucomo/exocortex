use derive_more::From;
use eframe::egui::{Modifiers, ViewportCommand};
use exocortex_keybinding::{KeyCommand, ShortcutState};

use self::Command::*;

#[derive(Clone, Debug, From)]
pub(crate) enum Command {
    Viewport(ViewportCommand),
    ViewportToggleFullscreen,
    CreateNewThop,
}

impl KeyCommand for Command {
    fn initialize_default_keymap(scs: &mut ShortcutState<Self>) {
        use ViewportCommand::Close;
        use eframe::egui::Key::{Enter, Escape, F};

        scs.define_command([(Escape, Modifiers::COMMAND)], Viewport(Close));
        scs.define_command([(F, Modifiers::COMMAND)], ViewportToggleFullscreen);
        scs.define_command([(Enter, Modifiers::COMMAND)], CreateNewThop);
    }
}
