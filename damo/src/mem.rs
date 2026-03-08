mod card;

use std::collections::BTreeMap;

use crate::errors::UnknownId;
use crate::{Id, Provider};

use self::card::MemCard;

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

    fn card_ref(&self, id: Id) -> Result<&MemCard, UnknownId> {
        self.cards.get(&id).ok_or(UnknownId(id))
    }

    fn card_mut(&mut self, id: Id) -> Result<&mut MemCard, UnknownId> {
        self.cards.get_mut(&id).ok_or(UnknownId(id))
    }
}

impl Provider for MemProvider {
    fn is_empty(&self) -> bool {
        self.nextid == 0
    }

    fn card_new<'p>(&mut self) -> Result<Id, UnknownId> {
        let id = self.alloc_id();
        self.cards.insert(id, MemCard::new());
        Ok(id)
    }

    fn card_prev(&self, optfrom: Option<Id>) -> Result<Option<Id>, UnknownId> {
        let id = optfrom.unwrap_or(self.nextid);
        Ok(id.checked_sub(1))
    }

    fn card_get_time_of_creation(&self, card: Id) -> Result<time::OffsetDateTime, UnknownId> {
        let card = self.card_ref(card)?;
        Ok(card.ctime)
    }

    fn card_get_synopsis(&self, card: Id) -> Result<&str, UnknownId> {
        let card = self.card_ref(card)?;
        Ok(card.synopsis.as_str())
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> Result<(), UnknownId> {
        let card = self.card_mut(card)?;
        card.synopsis = synopsis.to_string();
        Ok(())
    }
}
