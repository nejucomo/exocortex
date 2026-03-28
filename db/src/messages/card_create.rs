use exocortex_redborm::{Id, OrmResult, Store};
use redb::WriteTransaction;

use crate::entities::CardV0;

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

impl Store for CardCreate {
    type KOV = Id<CardV0>;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        CardV0.store_into(txn)
    }
}
