use exocortex_redborm::{OrmResult, Store};
use redb::WriteTransaction;

use crate::Id;
use crate::entities::Card;

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

impl Store for CardCreate {
    type KOV = Id<Card>;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        Card.store_into(txn)
    }
}
