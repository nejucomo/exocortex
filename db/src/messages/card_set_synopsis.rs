use derive_more::{From, Into};
use derive_new::new;
use redb::{TableDefinition, WriteTransaction};

use crate::dbio::{StoreColumnar, Tabular};
use crate::entities::Card;
use crate::{Id, Result, tables};

/// A request to set a card synopsis
#[derive(Debug, From, Into, new)]
pub struct CardSetSynopsis {
    pub card: Id<Card>,
    pub synopsis: String,
}

impl Tabular for CardSetSynopsis {
    type IdType = CardSetSynopsis;
    type RedVal = (Id<Card>, String);

    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::RedVal> {
        tables::CARD_SET_SYNOPSIS_V0
    }
}

impl StoreColumnar for CardSetSynopsis {
    type RedValStore = (Id<Card>, String);

    fn store_columnar(self, _: &WriteTransaction) -> Result<Self::RedValStore> {
        Ok(self.into())
    }
}
