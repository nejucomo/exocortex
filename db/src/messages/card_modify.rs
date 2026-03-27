//! Modify request type
use derive_more::{From, TryInto};
use redb::TableDefinition;

use crate::entities::Card;
use crate::messages::{CardCreate, CardSetSynopsis};
use crate::tables::{self, EnumColumnar};
use crate::{Id, Timestamp};

/// A request to modify cards
#[derive(Debug, From, TryInto)]
pub enum CardModify {
    #[allow(missing_docs)]
    Create(CardCreate),
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

/// A record of card modification
#[derive(Debug, From, TryInto)]
pub enum CardModified {
    #[allow(missing_docs)]
    Created(Id<Card>),
    #[allow(missing_docs)]
    SynopsisSet(CardSetSynopsis),
}

impl CardModified {
    pub fn card(&self) -> Id<Card> {
        use CardModified::*;

        match self {
            Created(card) => *card,
            SynopsisSet(css) => css.card,
        }
    }
}

impl Tabular for CardModify {
    type IdType = CardModify;
    type RedVal = (Timestamp, EnumColumnar);

    fn table_definition() -> TableDefinition<'static, Id<Self::IdType>, Self::RedVal> {
        tables::CARD_MODIFY_V0
    }
}

impl LoadColumnar for CardModified {
    type RedValStore = 

    fn store_columnar(
        self,
        txn: &redb::WriteTransaction,
    ) -> Result<<Self::RedValStore as redb::Value>::SelfType<'static>> {
    }
}
