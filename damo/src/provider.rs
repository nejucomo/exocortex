use enum_dispatch::enum_dispatch;
use time::OffsetDateTime;

use crate::errors::UnknownId;
use crate::{CanopyProvider, Id, MemProvider, MultiProvider};

/// A data model instance
///
/// The operations here aim to correspond to minimally human-meaningful actions. For example, this interface updates a Card's synopsis at once, so it doesn't include the detail of editing the synopsis (key strokes, voice input, etc...).
#[enum_dispatch]
pub trait Provider {
    /// Whether or not this provider contains any state
    fn is_empty(&self) -> bool;

    /// Create a new blank card
    fn card_new(&mut self) -> Result<Id, UnknownId>;

    /// Return the card just prior to `optfrom`, or the most recent card if [None]
    fn card_prev(&self, optfrom: Option<Id>) -> Result<Option<Id>, UnknownId>;

    /// Get the creation time
    fn card_get_time_of_creation(&self, card: Id) -> Result<OffsetDateTime, UnknownId>;

    /// Get the synopsis
    fn card_get_synopsis(&self, card: Id) -> Result<&str, UnknownId>;

    /// Set the synopsis
    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> Result<(), UnknownId>;
}
