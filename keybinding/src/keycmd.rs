use std::fmt::Debug;

use crate::ShortcutState;

/// The app's [KeyCommand] type defines key-bindable commands
pub trait KeyCommand: Clone + Debug {
    /// Initialize new keybindings via [ShortcutState::define_command]]
    fn initialize_default_keymap(scs: &mut ShortcutState<Self>);
}
