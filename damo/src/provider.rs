use time::OffsetDateTime;

/// A data model instance
///
/// The operations here aim to correspond to minimally human-meaningful actions. For example, this interface updates a Card's synopsis at once, so it doesn't include the detail of editing the synopsis (key strokes, voice input, etc...).
pub trait Provider: Sized + ProviderErrors {
    /// Whether or not this provider contains any state
    fn is_empty(&self) -> bool;

    /// A identifier for cards
    type CardId: Identifier;

    /// A card handle
    type Card: Card<UpdateError = Self::UpdateError, QueryError = Self::QueryError>;

    // A handle to a stack
    // type Stack: Stack;

    /// Create a new blank card
    fn new_card(&mut self) -> Result<Self::CardId, Self::UpdateError>;

    /// Return the card just prior to `optfrom`, or the most recent card if [None]
    fn prev_card(
        &self,
        optfrom: Option<Self::CardId>,
    ) -> Result<Option<Self::CardId>, Self::QueryError>;

    /// Open a card reference
    fn open_card_ref(&self, id: Self::CardId) -> Result<&Self::Card, Self::QueryError>;

    /// Open a mutable card reference
    fn open_card_mut(&mut self, id: Self::CardId) -> Result<&mut Self::Card, Self::UpdateError>;
}

/// The base of [Provider] which unifies the errors across updates and queries
pub trait ProviderErrors {
    /// The error type for updates
    type UpdateError: std::error::Error;
    /// The error type for queries
    type QueryError: std::error::Error;
}

/// A card-specific sub-API
pub trait Card: ProviderErrors {
    /// Get the creation time
    fn get_time_of_creation(&self) -> Result<OffsetDateTime, Self::QueryError>;

    /// Get the synopsis
    fn get_synopsis(&self) -> Result<&str, Self::QueryError>;

    /// Set the synopsis
    fn set_synopsis(&mut self, synopsis: &str) -> Result<(), Self::UpdateError>;
}

/// An identifier: each value is unique across a provider's persistent scope
pub trait Identifier: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::hash::Hash {}

impl<B> Identifier for B where B: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::hash::Hash {}
