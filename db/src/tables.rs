use redb::{MultimapTableDefinition, TableDefinition};

use crate::messages::{CardCreate, Modify};
use crate::{Id, Timestamp};

macro_rules! def_table {
    ( $name:ident, $tabdef:ident, $key:ty, $value:ty ) => {
        pub(crate) const $name: $tabdef<'static, $key, $value> = $tabdef::new(stringify!($name));
    };
}

def_table!(LOG_V0, MultimapTableDefinition, Timestamp, Id<Modify>);
def_table!(MODIFY_V0, TableDefinition, Id<Modify>, ());
def_table!(CARD_CREATE_V0, TableDefinition, Id<CardCreate>, ());
