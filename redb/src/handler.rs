use std::sync::Arc;

use redb::{Database, ReadableTableMetadata as _, WriteTransaction};

use crate::messages::{
    Card, CardAction, CardCreate, CardModification, CardModify, CardSetSynopsis, CardUpdated,
    Reply, Request,
};
use crate::tables::TABLES;
use crate::{Id, IdTagged};

/// The Result of DB operations
type HResult<T> = Result<T, redb::Error>;

pub(crate) trait Handler<R> {
    type Reply;

    fn handle(&mut self, request: R) -> HResult<Self::Reply>;
}

impl Handler<Request> for Database {
    type Reply = Reply;

    fn handle(
        &mut self,
        IdTagged {
            id: reqid,
            tagged: action,
        }: IdTagged<CardAction>,
    ) -> HResult<Self::Reply> {
        let updated = self.handle(action)?;

        Ok(Reply { reqid, updated })
    }
}

impl Handler<CardAction> for Database {
    type Reply = CardUpdated;

    fn handle(&mut self, action: CardAction) -> HResult<Self::Reply> {
        use CardAction::*;
        use CardUpdated::*;

        let mut txn = self.begin_write()?;
        match action {
            Create(x) => txn.handle(x).map(Created),
            Modify(x) => txn.handle(x).map(Modified),
        }
    }
}

impl Handler<CardCreate> for WriteTransaction {
    type Reply = Id<Card>;

    fn handle(&mut self, _: CardCreate) -> HResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        let card = tab.len().map(Id::new)?;
        tab.insert(card, "")?;
        Ok(card)
    }
}

impl Handler<Arc<CardModify>> for WriteTransaction {
    type Reply = Arc<CardModify>;

    fn handle(&mut self, acm: Arc<CardModify>) -> HResult<Self::Reply> {
        let CardModify { card, modif } = acm.as_ref();
        self.handle((*card, modif))?;
        Ok(acm)
    }
}

impl Handler<(Id<Card>, &CardModification)> for WriteTransaction {
    type Reply = ();

    fn handle(&mut self, (card, cmod): (Id<Card>, &CardModification)) -> HResult<Self::Reply> {
        use CardModification::*;

        match cmod {
            SetSynopsis(ss) => self.handle((card, ss)),
        }
    }
}

impl Handler<(Id<Card>, &CardSetSynopsis)> for WriteTransaction {
    type Reply = ();

    fn handle(
        &mut self,
        (card, CardSetSynopsis(syn)): (Id<Card>, &CardSetSynopsis),
    ) -> HResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        tab.insert(card, syn.as_str())?;
        Ok(())
    }
}
