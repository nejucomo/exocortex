use exocortex_redborm::{OrmResult, Store};
use redb::WriteTransaction;

use crate::CardId;
use crate::entities::CardV0;

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

impl Store for CardCreate {
    type KOV = CardId;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        CardV0.store_into(txn)
    }
}
