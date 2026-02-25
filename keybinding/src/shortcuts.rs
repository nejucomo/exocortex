use egui::KeyboardShortcut;
use thiserror::Error;

use crate::keymap::{KeyMap, KeyMapNode};

/// Manage tracking keymap command input sequences
#[derive(Debug, Default)]
pub struct ShortcutState<C: Clone> {
    keymap: KeyMap<C>,
    current: Option<KeyMap<C>>,
}

/// The result of attempting to handle a key
#[derive(Debug)]
pub enum HandleKey<C> {
    /// The key is a valid command prefix
    Pending,
    /// The key was the final key in the sequence for `C`
    Command(C),
    /// There have been no valid key presses previously and the key was unrecognized
    Unhandled(KeyboardShortcut),
}

/// The input state was pending from consuming earlier presses, but the key produces an unknown sequence
#[derive(Debug, Error)]
#[error("unknown key sequence")]
pub struct UnknownSequence;

impl<C> ShortcutState<C>
where
    C: Clone,
{
    pub fn define_command<I>(&mut self, keys: I, command: C) -> usize
    where
        I: IntoIterator<Item = KeyboardShortcut>,
    {
        self.keymap.define_command(keys, command)
    }

    /// Handle a `key`
    ///
    /// # Errors
    ///
    /// - `Err(())` - Some prior keys were consumed, and `key` is an invalid next key
    pub fn handle_key(&mut self, key: KeyboardShortcut) -> Result<HandleKey<C>, UnknownSequence> {
        use HandleKey::*;

        let (top_level, current) = if let Some(c) = self.current.as_ref() {
            (false, c)
        } else {
            (true, &self.keymap)
        };

        match current.match_key(key) {
            None if top_level => Ok(Unhandled(key)),
            None => Err(UnknownSequence),
            Some(KeyMapNode::Command(cmd)) => Ok(Command(cmd)),
            Some(KeyMapNode::Submap(submap)) => {
                self.current = Some(submap.clone());
                Ok(Pending)
            }
        }
    }
}
