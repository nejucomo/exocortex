//! [RowValue](exocortex_redborm::RowValue) types with [`Id`]`<Self>` keys

mod card;

pub trait Entity: RowValue<Key = Id<Self>> {}

use exocortex_redborm::RowValue;

use crate::Id;

pub use self::card::Card;
