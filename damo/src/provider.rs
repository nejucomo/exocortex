use crate::{CardView, DamoResult, Id};

/// A data model instance
///
/// The operations here aim to correspond to minimally human-meaningful actions. For example, this interface updates a Card's synopsis at once, so it doesn't include the detail of editing the synopsis (key strokes, voice input, etc...).
pub trait Provider {
    /// Whether or not this provider contains any state
    fn is_empty(&self) -> DamoResult<bool>;

    /// Create a new blank card
    fn card_new(&mut self) -> DamoResult<Id>;

    /// Set the synopsis
    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()>;

    /// An iterator over [CardView]
    type CardScan<'a>: Iterator<Item = DamoResult<CardView<'a>>>
    where
        Self: 'a;

    /// Scan cards
    fn card_scan(&self) -> DamoResult<Self::CardScan<'_>>;
}
