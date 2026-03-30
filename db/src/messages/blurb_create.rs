use exocortex_redborm::{OrmResult, Store};
use redb::WriteTransaction;

use crate::BlurbId;
use crate::entities::BlurbV0;

/// A request to create a new blurb
#[derive(Copy, Clone, Debug)]
pub struct BlurbCreate;

impl Store for BlurbCreate {
    type KOV = BlurbId;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        BlurbV0.store_into(txn)
    }
}
