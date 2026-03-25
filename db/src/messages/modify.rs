//! Modify request type
use derive_more::{From, TryInto};

use crate::Id;
use crate::entities::Card;

use crate::messages::{CardModify, Request};

/// A request to modify cards
#[derive(Debug, From, TryInto)]
pub enum Modify {
    #[allow(missing_docs)]
    #[from(CardCreate)]
    CardCreate(CardCreate),
    #[allow(missing_docs)]
    #[from(CardModify)]
    CardModify(CardModify),
}

impl Request for Modify {
    type Reply = Id<Card>;
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;
