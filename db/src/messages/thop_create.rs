use exocortex_redborm::{OrmResult, Store};
use redb::WriteTransaction;

use crate::ThopId;
use crate::entities::ThopV0;

/// A request to create a new thop
#[derive(Copy, Clone, Debug)]
pub struct ThopCreate;

impl Store for ThopCreate {
    type KOV = ThopId;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        ThopV0.store_into(txn)
    }
}
