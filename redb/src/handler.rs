use std::sync::Arc;

use redb::{
    Database, ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction,
};

use crate::messages::{
    Card, CardCreate, CardModification, CardModify, CardSetSynopsis, CardUpdated, DbIsEmpty,
    Modify, Queried, Query, RepSpec, Reply, ReqSpec, Request,
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
            tagged: reqspec,
        }: Request,
    ) -> HResult<Self::Reply> {
        let repspec = self.handle(reqspec)?;

        Ok(Reply { reqid, repspec })
    }
}

impl Handler<ReqSpec> for Database {
    type Reply = RepSpec;

    fn handle(&mut self, request: ReqSpec) -> HResult<Self::Reply> {
        use RepSpec::*;
        use ReqSpec::*;

        match request {
            Query(q) => {
                let mut txn = self.begin_read()?;
                txn.handle(q).map(Queried)
            }

            Modify(m) => {
                let mut txn = self.begin_write()?;
                txn.handle(m).map(Modified)
            }
        }
    }
}

impl Handler<Query> for ReadTransaction {
    type Reply = Queried;

    fn handle(&mut self, q: Query) -> HResult<Self::Reply> {
        use Queried::*;
        use Query::*;

        match q {
            DbIsEmpty(x) => self.handle(x).map(DbWasEmpty),
        }
    }
}

impl Handler<DbIsEmpty> for ReadTransaction {
    type Reply = bool;

    fn handle(&mut self, _: DbIsEmpty) -> HResult<Self::Reply> {
        let tab = self.open_table(TABLES.card_synopsis)?;
        let len = tab.len()?;
        Ok(len == 0)
    }
}

impl Handler<Modify> for WriteTransaction {
    type Reply = CardUpdated;

    fn handle(&mut self, action: Modify) -> HResult<Self::Reply> {
        use CardUpdated::*;
        use Modify::*;

        match action {
            CardCreate(x) => self.handle(x).map(Created),
            CardModify(x) => self.handle(x).map(Modified),
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
