use thiserror::Error;

use crate::keymap::{KeyMap, KeyMapNode};
use crate::{KeyChord, KeyCommand};

/// Manage tracking keymap command input sequences
#[derive(Debug)]
pub struct ShortcutState<C: KeyCommand> {
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
    Unhandled(KeyChord),
}

/// The input state was pending from consuming earlier presses, but the key produces an unknown sequence
#[derive(Debug, Error)]
#[error("unknown key sequence")]
pub struct UnknownSequence;

impl<C> Default for ShortcutState<C>
where
    C: KeyCommand,
{
    fn default() -> Self {
        let mut scs = Self {
            keymap: KeyMap::default(),
            current: None,
        };

        C::initialize_default_keymap(&mut scs);
        scs
    }
}

impl<C> ShortcutState<C>
where
    C: KeyCommand,
{
    /// Define a command, panic if this overwrites any existing binding
    pub fn define_command<I, K>(&mut self, keys: I, command: C)
    where
        I: IntoIterator<Item = K>,
        K: Into<KeyChord>,
    {
        let overwritten = self.overwrite_command(keys, command);
        assert_eq!(0, overwritten);
    }

    /// Define a command, overwriting if necessary
    pub fn overwrite_command<I, K>(&mut self, keys: I, command: C) -> usize
    where
        I: IntoIterator<Item = K>,
        K: Into<KeyChord>,
    {
        self.keymap
            .define_command(keys.into_iter().map(|k| k.into()), command)
    }

    /// Handle a `key`
    ///
    /// # Errors
    ///
    /// - `Err(())` - Some prior keys were consumed, and `key` is an invalid next key
    pub fn handle_key<K>(&mut self, key: K) -> Result<HandleKey<C>, UnknownSequence>
    where
        K: Into<KeyChord>,
    {
        use HandleKey::*;

        let (top_level, current) = if let Some(c) = self.current.as_ref() {
            (false, c)
        } else {
            (true, &self.keymap)
        };

        let key = key.into();
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
