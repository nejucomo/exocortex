//! Message types sent to/from the DB thread
use std::sync::Arc;

use crate::{Id, IdTagged};

/// The top-level request sent by applications to the DB thread
pub type Request = IdTagged<CardAction>;

/// A request to modify cards
#[derive(Debug)]
pub enum CardAction {
    #[allow(missing_docs)]
    Create(CardCreate),
    #[allow(missing_docs)]
    Modify(Arc<CardModify>),
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

/// A request to modify a specific card
#[derive(Debug)]
pub struct CardModify {
    #[allow(missing_docs)]
    pub card: Id<Card>,
    #[allow(missing_docs)]
    pub modif: CardModification,
}

/// A requested modification of a card
#[derive(Debug)]
pub enum CardModification {
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

/// A request to set a card synopsis
#[derive(Debug)]
pub struct CardSetSynopsis(pub String);

/// The top-level reply sent from the DB thread to applications
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Reply {
    #[allow(missing_docs)]
    pub reqid: Id<CardAction>,
    #[allow(missing_docs)]
    pub updated: CardUpdated,
}

/// A reply about a successful update to a card
#[derive(Debug)]
pub enum CardUpdated {
    #[allow(missing_docs)]
    Created(Id<Card>),
    #[allow(missing_docs)]
    Modified(Arc<CardModify>),
}

/// A type-disambiguation placeholder for `Id<Card>`
#[derive(Copy, Clone, Debug)]
pub enum Card {}
