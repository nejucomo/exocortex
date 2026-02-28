//! Queries to which [Provider](crate::Provider)s respond
use derive_new::new;
use time::OffsetDateTime;

use crate::{Id, Query};

/// Get the creation time of a card
#[derive(Debug, new)]
pub struct GetTimeOfCreation(pub Id);

impl Query for GetTimeOfCreation {
    type Answer<'p> = OffsetDateTime;
}

/// Get the current synopsis of a card
#[derive(Debug, new)]
pub struct GetSynopsis(pub Id);

impl Query for GetSynopsis {
    type Answer<'p> = &'p str;
}
