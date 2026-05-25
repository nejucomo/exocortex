use derive_more::From;
use eframe::egui::Modifiers;
use eframe::egui::ViewportCommand::{self, Close};
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
        use eframe::egui::Key::{Enter, Escape, F, Slash};

        let appmod = Modifiers::COMMAND;
        scs.define_command([(Slash, appmod)], ShortcutsHelp);
        scs.define_command([(Escape, appmod)], Viewport(Close));
        scs.define_command([(F, appmod)], ViewportToggleFullscreen);
        scs.define_command([(Enter, appmod)], CreateNewThop);
    }

    fn help(&self) -> &'static str {
        match self {
            ShortcutsHelp => "display keyboard shortcuts help",
            Viewport(Close) => concat!("close ", env!("CARGO_PKG_NAME")),
            Viewport(vpc) => todo!("help unimplemented for {vpc:?}"),
            ViewportToggleFullscreen => "toggle fullscreen mode",
            CreateNewThop => "append a new thop",
        }
    }
}
