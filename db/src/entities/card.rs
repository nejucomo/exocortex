use exocortex_redborm::RowValue;
use redb_derive::Value;

use crate::Id;

/// The [Card] entity
#[derive(Copy, Clone, Debug, Value)]
pub struct CardV0;

impl RowValue for Card {
    type Key = Id<Self>;
}
