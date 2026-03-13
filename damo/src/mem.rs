mod card;
mod scan;

use std::collections::BTreeMap;

use crate::{DamoError::UnknownId, DamoResult, Id, Provider};

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

    // fn card_ref(&self, id: Id) -> DamoResult<&MemCard> {
    //     self.cards.get(&id).ok_or(UnknownId(id))
    // }

    fn card_mut(&mut self, id: Id) -> DamoResult<&mut MemCard> {
        self.cards.get_mut(&id).ok_or(UnknownId(id))
    }
}

impl Provider for MemProvider {
    fn is_empty(&self) -> DamoResult<bool> {
        Ok(self.nextid == 0)
    }

    fn card_new<'p>(&mut self) -> DamoResult<Id> {
        let id = self.alloc_id();
        self.cards.insert(id, MemCard::new());
        Ok(id)
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()> {
        let card = self.card_mut(card)?;
        card.synopsis = synopsis.to_string();
        Ok(())
    }

    type CardScan<'a>
        = scan::CardScan<'a>
    where
        Self: 'a;

    fn card_scan(&self) -> DamoResult<Self::CardScan<'_>> {
        Ok(scan::CardScan::new(self.cards.iter()))
    }
}
