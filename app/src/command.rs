use derive_more::From;
use eframe::egui::{Modifiers, ViewportCommand};
use exocortex_keybinding::{KeyCommand, ShortcutState};

use self::Command::*;

#[derive(Clone, Debug, From)]
pub(crate) enum Command {
    ShortcutsHelp,
    Viewport(ViewportCommand),
    ViewportToggleFullscreen,
    CreateNewThop,
}

impl KeyCommand for Command {
    fn initialize_default_keymap(scs: &mut ShortcutState<Self>) {
        use ViewportCommand::Close;
        use eframe::egui::Key::{Enter, Escape, F, Slash};

        let appmod = Modifiers::COMMAND;
        scs.define_command([(Slash, appmod)], ShortcutsHelp);
        scs.define_command([(Escape, appmod)], Viewport(Close));
        scs.define_command([(F, appmod)], ViewportToggleFullscreen);
        scs.define_command([(Enter, appmod)], CreateNewThop);
    }
}
