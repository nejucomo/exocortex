use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::KeyChord;

/// A mapping from key sequences to app commands `C`
#[derive(Clone, Debug)]
pub struct KeyMap<C: Clone>(Rc<RefCell<HashMap<KeyChord, KeyMapNode<C>>>>);

#[derive(Clone, Debug)]
pub(crate) enum KeyMapNode<C: Clone> {
    Command(C),
    Submap(KeyMap<C>),
}

impl<C> Default for KeyMap<C>
where
    C: Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C> KeyMap<C>
where
    C: Clone,
{
    /// Insert a command, returning the number of prior keybindings overwritten
    pub(crate) fn define_command<I>(&mut self, keys: I, command: C) -> usize
    where
        I: IntoIterator<Item = KeyChord>,
    {
        let mut keys = keys.into_iter();
        let key = keys.next().unwrap();
        self.define_command_inner(key, keys, command)
    }

    fn define_command_inner<I>(&mut self, key: KeyChord, keys: I, command: C) -> usize
    where
        I: Iterator<Item = KeyChord>,
    {
        let mut hm = self.0.borrow_mut();

        if let Some(node) = hm.get_mut(&key) {
            node.define_command(keys, command)
        } else {
            hm.insert(key, KeyMapNode::new(keys, command));
            0
        }
    }

    /// Attempt to match `key`
    pub(crate) fn match_key(&self, key: KeyChord) -> Option<KeyMapNode<C>> {
        self.0.borrow().get(&key).cloned()
    }
}

impl<C> KeyMapNode<C>
where
    C: Clone,
{
    fn new<I>(mut keys: I, command: C) -> Self
    where
        I: Iterator<Item = KeyChord>,
    {
        use KeyMapNode::*;

        if let Some(key) = keys.next() {
            let mut submap = KeyMap::default();
            let n = submap.define_command_inner(key, keys, command);
            assert_eq!(0, n);
            Submap(submap)
        } else {
            Command(command)
        }
    }

    fn define_command<I>(&mut self, mut keys: I, command: C) -> usize
    where
        I: Iterator<Item = KeyChord>,
    {
        use KeyMapNode::*;

        let (optnewself, overwritten) = match (&mut *self, keys.next()) {
            (Command(_), None) => (Some(Command(command)), 1),

            (Command(_), Some(key)) => {
                let mut submap = KeyMap::default();
                let n = submap.define_command_inner(key, keys, command);
                assert_eq!(0, n);
                (Some(Submap(submap)), 1)
            }

            (Submap(key_map), None) => {
                let overwritten = key_map.0.borrow().len();
                (Some(Command(command)), overwritten)
            }

            (Submap(key_map), Some(key)) => {
                let n = key_map.define_command_inner(key, keys, command);
                (None, n)
            }
        };

        if let Some(newself) = optnewself {
            *self = newself;
        }
        overwritten
    }
}
