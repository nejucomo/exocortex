use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::node::Node;
use crate::{KeyChord, KeyCommand};

/// A mapping from key sequences to app commands `C`
#[derive(Clone, Debug)]
pub(crate) struct KeyMap<C: KeyCommand>(Rc<RefCell<HashMap<KeyChord, Node<C>>>>);

impl<C> Default for KeyMap<C>
where
    C: KeyCommand,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C> KeyMap<C>
where
    C: KeyCommand,
{
    pub(crate) fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Insert a command, returning the number of prior keybindings overwritten
    pub(crate) fn define_command<I>(&mut self, keys: I, command: C) -> usize
    where
        I: IntoIterator<Item = KeyChord>,
    {
        let mut keys = keys.into_iter();
        let key = keys.next().unwrap();
        self.define_command_inner(key, keys, command)
    }

    pub(crate) fn define_command_inner<I>(&mut self, key: KeyChord, keys: I, command: C) -> usize
    where
        I: Iterator<Item = KeyChord>,
    {
        let mut hm = self.0.borrow_mut();

        if let Some(node) = hm.get_mut(&key) {
            node.define_command(keys, command)
        } else {
            hm.insert(key, Node::new(keys, command));
            0
        }
    }

    /// Attempt to match `key`
    pub(crate) fn match_key(&self, key: KeyChord) -> Option<Node<C>> {
        self.0.borrow().get(&key).cloned()
    }
}
