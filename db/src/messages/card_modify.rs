//! CardModify request type
use derive_more::{From, TryInto};
use derive_new::new;

use crate::Id;
use crate::entities::Card;

use crate::messages::Request;

/// A request to modify a specific card
#[derive(Debug, From, new)]
pub struct CardModify {
    #[allow(missing_docs)]
    pub card: Id<Card>,
    #[allow(missing_docs)]
    #[new(into)]
    pub modif: CardModification,
}

impl Request for CardModify {
    type Reply = Id<Card>;
}

/// A requested modification of a card
#[derive(Debug, From, TryInto)]
pub enum CardModification {
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

/// A request to set a card synopsis
#[derive(Debug, From, new)]
pub struct CardSetSynopsis(pub String);
