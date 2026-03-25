//! Modify request type
use derive_more::{From, TryInto};
use derive_new::new;
use redb::WriteTransaction;

use crate::entities::Card;
use crate::messages::Request;
use crate::storeload::Store;
use crate::{Id, Result, Timestamped};

/// A request to modify cards
#[derive(Debug, From, TryInto)]
pub enum CardModify {
    #[allow(missing_docs)]
    CardCreate(CardCreate),
    #[allow(missing_docs)]
    CardSetSynopsis(CardSetSynopsis),
}

impl Request for CardModify {
    type Reply = (Id<CardModify>, CardModified);
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

/// A request to set a card synopsis
#[derive(Debug, From, new)]
pub struct CardSetSynopsis {
    pub card: Id<Card>,
    pub synopsis: String,
}

/// A record of card modification
#[derive(Debug, From, TryInto)]
pub enum CardModified {
    #[allow(missing_docs)]
    CardCreated(Id<Card>),
    #[allow(missing_docs)]
    CardSetSynopsis(CardSetSynopsis),
}

impl CardModified {
    pub fn card(&self) -> Id<Card> {
        use CardModified::*;

        match self {
            CardCreated(card) => *card,
            CardSetSynopsis(css) => css.card,
        }
    }
}

impl Store for CardModify {
    type Stored = Timestamped<CardModified>;

    fn store_into(self, txn: &WriteTransaction) -> Result<(Id<Self>, Self::Stored)> {
        todo!()
    }
}
