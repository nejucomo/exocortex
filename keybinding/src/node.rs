use crate::keymap::KeyMap;
use crate::{KeyChord, KeyCommand};

#[derive(Clone, Debug)]
pub(crate) enum Node<C: KeyCommand> {
    Command(C),
    Submap(KeyMap<C>),
}

impl<C> Node<C>
where
    C: KeyCommand,
{
    pub(crate) fn new<I>(mut keys: I, command: C) -> Self
    where
        I: Iterator<Item = KeyChord>,
    {
        use Node::*;

        if let Some(key) = keys.next() {
            let mut submap = KeyMap::default();
            let n = submap.define_command_inner(key, keys, command);
            assert_eq!(0, n);
            Submap(submap)
        } else {
            Command(command)
        }
    }

    pub(crate) fn define_command<I>(&mut self, mut keys: I, command: C) -> usize
    where
        I: Iterator<Item = KeyChord>,
    {
        use Node::*;

        let (optnewself, overwritten) = match (&mut *self, keys.next()) {
            (Command(_), None) => (Some(Command(command)), 1),

            (Command(_), Some(key)) => {
                let mut submap = KeyMap::default();
                let n = submap.define_command_inner(key, keys, command);
                assert_eq!(0, n);
                (Some(Submap(submap)), 1)
            }

            (Submap(key_map), None) => {
                let overwritten = key_map.len();
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
