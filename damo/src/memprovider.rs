mod card;

use std::collections::BTreeMap;

use thiserror::Error;
use time::OffsetDateTime;

use crate::queries::{GetSynopsis, GetTimeOfCreation};
use crate::updates::SetSynopsis;
use crate::{Id, Provider, ProviderBase, Queryable, Update};

use self::card::MemCard;

#[derive(Debug, Error)]
#[error("Unknown {:?}", .0)]
pub struct UnknownId(Id);

/// An ephemral [Provider] backed by runtime memory
#[derive(Debug)]
pub struct MemoryProvider {
    nextid: Id,
    cards: BTreeMap<Id, MemCard>,
}

impl Default for MemoryProvider {
    fn default() -> Self {
        Self {
            nextid: Id::from(0),
            cards: BTreeMap::default(),
        }
    }
}

impl ProviderBase for MemoryProvider {
    type UpdateError = UnknownId;
    type QueryError = UnknownId;
}

impl Provider for MemoryProvider {
    fn new_card(&mut self) -> Result<Id, Self::UpdateError> {
        let id = self.nextid;
        self.nextid = Id::new(self.nextid.into_u64() + 1);
        let card = MemCard::new();
        self.cards.insert(id, card);
        Ok(id)
    }
}

impl<'a> Update<SetSynopsis<'a>> for MemoryProvider {
    fn update(&mut self, request: SetSynopsis<'a>) -> Result<(), Self::UpdateError> {
        let SetSynopsis { card, synopsis } = request;
        let card = self.get_card_mut(card)?;
        card.synopsis = synopsis.to_string();
        Ok(())
    }
}

impl Queryable<GetTimeOfCreation> for MemoryProvider {
    fn query(
        &self,
        GetTimeOfCreation(id): GetTimeOfCreation,
    ) -> Result<OffsetDateTime, Self::QueryError> {
        self.get_card(id).map(|c| c.ctime)
    }
}

impl Queryable<GetSynopsis> for MemoryProvider {
    fn query<'p>(&'p self, GetSynopsis(id): GetSynopsis) -> Result<&'p str, Self::QueryError> {
        self.get_card(id).map(|c| c.synopsis.as_str())
    }
}

impl MemoryProvider {
    fn get_card(&self, id: Id) -> Result<&MemCard, UnknownId> {
        self.cards.get(&id).ok_or(UnknownId(id))
    }

    fn get_card_mut(&mut self, id: Id) -> Result<&mut MemCard, UnknownId> {
        self.cards.get_mut(&id).ok_or(UnknownId(id))
    }
}
