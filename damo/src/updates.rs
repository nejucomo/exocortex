//! Updates which [Provider](crate::Provider)s fulfill
use derive_new::new;

use crate::Id;

/// Set/overwrite the synopsis of a card
#[derive(Debug, new)]
pub struct SetSynopsis<'a> {
    /// The card to update
    pub card: Id,
    /// The new synopsis
    pub synopsis: &'a str,
}
