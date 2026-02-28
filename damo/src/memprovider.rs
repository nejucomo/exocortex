use std::collections::BTreeMap;

use thiserror::Error;

use crate::queries::GetSynopsis;
use crate::updates::SetSynopsis;
use crate::{Id, Provider, ProviderBase, Query, Update};

/// An ephemral [Provider] backed by runtime memory
#[derive(Debug)]
pub struct MemoryProvider {
    nextid: Id,
    synopsis: BTreeMap<Id, String>,
}

impl Default for MemoryProvider {
    fn default() -> Self {
        Self {
            nextid: Id::from(0),
            synopsis: BTreeMap::default(),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unknown {:?}", .0)]
pub struct UnknownId(Id);

impl ProviderBase for MemoryProvider {
    type UpdateError = UnknownId;
    type QueryError = UnknownId;
}

impl Provider for MemoryProvider {
    fn new_card(&mut self) -> Result<Id, Self::UpdateError> {
        let id = self.nextid;
        self.nextid = Id::new(self.nextid.into_u64() + 1);
        Ok(id)
    }
}

impl<'a> Update<SetSynopsis<'a>> for MemoryProvider {
    fn update(&mut self, request: SetSynopsis<'a>) -> Result<(), Self::UpdateError> {
        let SetSynopsis { card, synopsis } = request;
        self.synopsis.insert(card, synopsis.to_string());
        Ok(())
    }
}

impl<'a> Query<'a, GetSynopsis> for MemoryProvider {
    type Answer = &'a str;

    fn query(&'a self, query: GetSynopsis) -> Result<Self::Answer, Self::QueryError> {
        let GetSynopsis(id) = query;
        self.synopsis
            .get(&id)
            .map(String::as_str)
            .ok_or(UnknownId(id))
    }
}
