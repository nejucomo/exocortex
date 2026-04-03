use exocortex_lid::Id;
use redb::TableDefinition;

use crate::{OwnedKey, OwnedValue};

/// A table key & row type for a specific table
///
/// # Blanket Store/Load
///
/// There are blanket impls of [Store](crate::Store) and [Load](crate::Load) for [RowValue], serving as a typical leaf-case for [Store](crate::Store)/[Load](crate::Load) composition.
pub trait RowValue: OwnedValue {
    /// The key type
    type Key: OwnedKey;

    /// The name of the table
    fn table_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// The table definition for this type
    fn table_definition() -> TableDefinition<'static, Self::Key, Self> {
        TableDefinition::new(Self::table_name())
    }
}

/// An [Entity] is any [RowValue] with [`Id`]`<Self>` keys
///
/// # Blanket impl
///
/// There is a blanket impl of [RowValue] for any [Entity], so declaring `impl Entity for Foo {}` is a sufficient declaration.
pub trait Entity: RowValue<Key = Id<Self>> {}

impl<B> RowValue for B
where
    B: Entity + OwnedValue,
{
    type Key = Id<Self>;
}
