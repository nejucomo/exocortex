use crate::dbio::{StoreColumnar, Tabular};
use crate::entities::Card;
use crate::{Id, Result, tables};

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

impl Tabular for CardCreate {
    type IdType = Card;
    type RedVal = ();

    fn table_definition() -> redb::TableDefinition<'static, Id<Card>, Self::RedVal> {
        tables::CARD_CREATE_V0
    }
}

impl StoreColumnar for CardCreate {
    type RedValStore = ();

    fn store_columnar(self, _: &redb::WriteTransaction) -> Result<()> {
        Ok(())
    }
}
