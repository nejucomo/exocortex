use std::collections::BTreeMap;

use exocortex_damo::{Provider, ProviderErrors};

use crate::{Id, MemCard, UnknownId};

/// An ephemral [Provider] backed by runtime memory
#[derive(Debug, Default)]
pub struct MemProvider {
    nextid: Id,
    cards: BTreeMap<Id, MemCard>,
}

impl MemProvider {
    fn alloc_id(&mut self) -> Id {
        let id = self.nextid;
        self.nextid += 1;
        id
    }
}

impl ProviderErrors for MemProvider {
    type UpdateError = UnknownId;
    type QueryError = UnknownId;
}

impl Provider for MemProvider {
    type CardId = Id;
    type Card = MemCard;

    fn new_card<'p>(&mut self) -> Result<Id, Self::UpdateError> {
        let id = self.alloc_id();
        self.cards.insert(id, MemCard::new());
        Ok(id)
    }

    fn open_card_ref(&self, id: Self::CardId) -> Result<&Self::Card, Self::QueryError> {
        self.cards.get(&id).ok_or(UnknownId(id))
    }

    fn open_card_mut(&mut self, id: Self::CardId) -> Result<&mut Self::Card, Self::UpdateError> {
        self.cards.get_mut(&id).ok_or(UnknownId(id))
    }
}
