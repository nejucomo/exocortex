//! Queries to which [Provider](crate::Provider)s respond
use derive_new::new;

use crate::Id;

/// Get the current synopsis of a card
#[derive(Debug, new)]
pub struct GetSynopsis(pub Id);
