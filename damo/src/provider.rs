use time::OffsetDateTime;

use crate::queries::{GetSynopsis, GetTimeOfCreation};
use crate::updates::SetSynopsis;
use crate::{Id, Queryable, Update};

/// A data model instance
///
/// The operations here aim to correspond to minimally human-meaningful actions. For example, this interface updates a Card's synopsis at once, so it doesn't include the detail of editing the synopsis (key strokes, voice input, etc...).
pub trait Provider:
    ProviderBase
    + for<'a> Update<SetSynopsis<'a>>
    + Queryable<GetTimeOfCreation>
    + Queryable<GetSynopsis>
{
    /// This is the only mutation which returns a result by allocating a new instance-unique [Id]
    fn new_card(&mut self) -> Result<Id, Self::UpdateError>;

    fn set_synopsis(&mut self, card: Id, synopsis: &str) -> Result<(), Self::UpdateError> {
        self.update(SetSynopsis { card, synopsis })
    }

    fn get_time_of_creation(&self, card: Id) -> Result<OffsetDateTime, Self::QueryError> {
        self.query(GetTimeOfCreation(card))
    }

    fn get_synopsis(&self, card: Id) -> Result<&str, Self::QueryError> {
        self.query(GetSynopsis(card))
    }
}

/// The base of [Provider] which unifies the errors across updates and queries
pub trait ProviderBase {
    /// The error type for updates
    type UpdateError;
    /// The error type for queries
    type QueryError;
}
