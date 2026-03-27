use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::RowValue;
use redb_derive::Value;

use crate::Id;
use crate::entities::Card;

/// A request to set a card synopsis
#[derive(Debug, From, Into, new, Value)]
pub struct CardSetSynopsis {
    pub card: Id<Card>,
    pub synopsis: String,
}

/// # TODO
///
/// Intro `Entity` trait extension of [RowValue] which sets the key to an id
impl RowValue for CardSetSynopsis {
    type Key = Id<CardSetSynopsis>;
}
