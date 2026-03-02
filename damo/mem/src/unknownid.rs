use thiserror::Error;

use crate::Id;

/// The only [MemProvider](crate::MemProvider) error case
#[derive(Debug, Error)]
#[error("Unknown {:?}", .0)]
pub struct UnknownId(pub Id);
