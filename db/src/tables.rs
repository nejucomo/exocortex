use redb::TableDefinition;

use crate::entities::Card;
use crate::messages::{CardCreate, CardModify, CardSetSynopsis, Modify};
use crate::{Id, Timestamp, Timestamped};

/// Every (normalized) enum value is a variant selector and the id of the variant table
///
/// The type `Id<Variant>` is a legibility hack.
pub(crate) type EnumValue = (Variant, Id<Variant>);

/// A unique value for each enum variant
pub(crate) type Variant = u32;

macro_rules! def_tables {
    ( $( $tabtype:ident { $name:ident : $key:ty => $value:ty } );* $(;)? ) => {
        $(
            def_tables!(@internal $tabtype, $name, $key, $value);
        )*
    };

    ( @internal $tabtype:ident, $name:ident, $key:ty, $value:ty ) => {
        pub(crate) const $name: $tabtype<'static, $key, $value> = $tabtype::new(stringify!($name));
    };
}

def_tables!(
    // MultimapTableDefinition { LOG_V0 : Timestamp => Id<Modify> };

    TableDefinition { MODIFY_V0 : Id<Timestamped<Modify>> => (Timestamp, EnumValue) };
    TableDefinition { CARD_CREATE_V0: Id<CardCreate> => () };
    TableDefinition { CARD_MODIFY_V0: Id<CardModify> => (Id<Card>, EnumValue) };
    TableDefinition { CARD_SET_SYNOPSIS_V0: Id<CardSetSynopsis> => String };
);
