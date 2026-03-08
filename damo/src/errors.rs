//! damo error types

use thiserror::Error;

use crate::Id;

/// An unknown [Id] was passed to the API
#[derive(Debug, Error)]
#[error("Unknown {:?}", .0)]
pub struct UnknownId(pub Id);
