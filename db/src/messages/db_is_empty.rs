//! DbIsEmpty request type
use crate::messages::Request;

/// A query if the db is empty (ie: newly created)
#[derive(Copy, Clone, Debug)]
pub struct DbIsEmpty;

impl Request for DbIsEmpty {
    type Reply = bool;
}
