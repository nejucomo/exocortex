use crate::queries::GetSynopsis;
use crate::updates::SetSynopsis;
use crate::{Id, Query, Update};

/// A data model instance
///
/// The operations here aim to correspond to minimally human-meaningful actions. For example, this interface updates a Card's synopsis at once, so it doesn't include the detail of editing the synopsis (key strokes, voice input, etc...).
pub trait Provider:
    ProviderBase + for<'a> Update<SetSynopsis<'a>> + for<'a> Query<'a, GetSynopsis, Answer = &'a str>
{
    /// This is the only mutation which returns a result by allocating a new instance-unique [Id]
    fn new_card(&mut self) -> Result<Id, Self::UpdateError>;
}

/// The base of [Provider] which unifies the errors across updates and queries
pub trait ProviderBase {
    /// The error type for updates
    type UpdateError;
    /// The error type for queries
    type QueryError;
}
