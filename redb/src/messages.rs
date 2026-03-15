//! Message types sent to/from the DB thread
use std::sync::Arc;

use derive_more::From;
use derive_new::new;

use crate::{Id, IdTagged};

/// The top-level request sent by applications to the DB thread
pub type Request = IdTagged<ReqSpec>;

/// A request to modify cards
#[derive(Debug, From)]
pub enum ReqSpec {
    #[allow(missing_docs)]
    #[from(Query, DbIsEmpty)]
    Query(Query),
    #[allow(missing_docs)]
    #[from(Modify, CardCreate, CardModify)]
    Modify(Modify),
}

/// A query of the db
#[derive(Debug, From)]
pub enum Query {
    #[allow(missing_docs)]
    DbIsEmpty(DbIsEmpty),
}

/// A query if the db is empty (ie: newly created)
#[derive(Copy, Clone, Debug)]
pub struct DbIsEmpty;

/// A request to modify cards
#[derive(Debug, From)]
pub enum Modify {
    #[allow(missing_docs)]
    #[from(CardCreate)]
    CardCreate(CardCreate),
    #[allow(missing_docs)]
    #[from(CardModify)]
    CardModify(Arc<CardModify>),
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

/// A request to modify a specific card
#[derive(Debug, From, new)]
pub struct CardModify {
    #[allow(missing_docs)]
    pub card: Id<Card>,
    #[allow(missing_docs)]
    #[new(into)]
    pub modif: CardModification,
}

/// A requested modification of a card
#[derive(Debug, From)]
pub enum CardModification {
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

/// A request to set a card synopsis
#[derive(Debug, From, new)]
pub struct CardSetSynopsis(pub String);

/// The top-level reply sent from the DB thread to applications
#[derive(Debug, From, new)]
pub struct Reply {
    /// The corresponding request id:
    pub reqid: Id<ReqSpec>,
    #[allow(missing_docs)]
    #[new(into)]
    pub repspec: RepSpec,
}

/// Reply specifics
#[derive(Debug, From)]
pub enum RepSpec {
    #[allow(missing_docs)]
    Queried(Queried),
    #[allow(missing_docs)]
    Modified(CardUpdated),
}

/// Reply specifics
#[derive(Debug, From)]
pub enum Queried {
    #[allow(missing_docs)]
    DbWasEmpty(bool),
}

/// A reply about a successful update to a card
#[derive(Debug, From)]
pub enum CardUpdated {
    #[allow(missing_docs)]
    Created(Id<Card>),
    #[allow(missing_docs)]
    Modified(Arc<CardModify>),
}

/// A type-disambiguation placeholder for `Id<Card>`
#[derive(Copy, Clone, Debug, From)]
pub enum Card {}
