use std::sync::Arc;

use redb::{
    Database, ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction,
};

use crate::Id;
use crate::channel::{FromApp, ToApp};
use crate::messages::{
    Card, CardCreate, CardModification, CardModify, CardScan, CardScanned, CardSetSynopsis,
    CardUpdated, DbIsEmpty, Modify, Queried, Query, RepSpec, Reply, ReqSpec, Request,
};
use crate::tables::TABLES;

pub(crate) fn run_db_thread(db: Database, to_from_app: (ToApp, FromApp)) {
    run_inner(db, to_from_app).unwrap()
}

fn run_inner(
    mut db: Database,
    (to_app, from_app): (ToApp, FromApp),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut scan: Option<Scan> = None;

    loop {
        let req = from_app.recv()?;
        let rep = (&mut db, &mut scan).handle(req)?;
        to_app.send(rep)?;
    }
}

/// The Result of DB operations
type HResult<T> = Result<T, redb::Error>;

type Scan = ();

type WithScan<'a, T> = (T, &'a mut Option<Scan>);

trait Handler<R> {
    type Reply;

    fn handle(self, request: R) -> HResult<Self::Reply>;
}

impl Handler<Request> for WithScan<'_, &mut Database> {
    type Reply = Reply;

    fn handle(self, Request { id, reqspec }: Request) -> HResult<Self::Reply> {
        let repspec = self.handle(reqspec)?;

        Ok(Reply { reqid: id, repspec })
    }
}

impl Handler<ReqSpec> for WithScan<'_, &mut Database> {
    type Reply = RepSpec;

    fn handle(self, request: ReqSpec) -> HResult<Self::Reply> {
        use RepSpec::*;
        use ReqSpec::*;

        let (db, scan) = self;

        match request {
            Query(q) => {
                let txn = db.begin_read()?;
                (txn, scan).handle(q).map(Queried)
            }

            Modify(m) => {
                let txn = db.begin_write()?;
                txn.handle(m).map(Modified)
            }
        }
    }
}

impl Handler<Query> for WithScan<'_, ReadTransaction> {
    type Reply = Queried;

    fn handle(self, q: Query) -> HResult<Self::Reply> {
        use Queried::*;
        use Query::*;

        match q {
            DbIsEmpty(x) => self.0.handle(x).map(DbWasEmpty),
            CardScan(x) => self.handle(x).map(CardScanned),
        }
    }
}

impl Handler<DbIsEmpty> for ReadTransaction {
    type Reply = bool;

    fn handle(self, _: DbIsEmpty) -> HResult<Self::Reply> {
        let tab = self.open_table(TABLES.card_synopsis)?;
        let len = tab.len()?;
        Ok(len == 0)
    }
}

impl Handler<CardScan> for WithScan<'_, ReadTransaction> {
    type Reply = CardScanned;

    fn handle(self, request: CardScan) -> HResult<Self::Reply> {
        use CardScan::*;
        use CardScanned::*;

        let (txn, scan) = self;

        match request {
            Next => {
                let _ = scan.get_or_insert_with(|| {
                    let _ = txn;
                    todo!()
                });
                todo!()
            }
            Stop => {
                *scan = None;
                Ok(Stopped)
            }
        }
    }
}

impl Handler<Modify> for WriteTransaction {
    type Reply = CardUpdated;

    fn handle(self, action: Modify) -> HResult<Self::Reply> {
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

    fn handle(self, _: CardCreate) -> HResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        let card = tab.len().map(Id::new)?;
        tab.insert(card, "")?;
        Ok(card)
    }
}

impl Handler<Arc<CardModify>> for WriteTransaction {
    type Reply = Arc<CardModify>;

    fn handle(self, acm: Arc<CardModify>) -> HResult<Self::Reply> {
        let CardModify { card, modif } = acm.as_ref();
        self.handle((*card, modif))?;
        Ok(acm)
    }
}

impl Handler<(Id<Card>, &CardModification)> for WriteTransaction {
    type Reply = ();

    fn handle(self, (card, cmod): (Id<Card>, &CardModification)) -> HResult<Self::Reply> {
        use CardModification::*;

        match cmod {
            SetSynopsis(ss) => self.handle((card, ss)),
        }
    }
}

impl Handler<(Id<Card>, &CardSetSynopsis)> for WriteTransaction {
    type Reply = ();

    fn handle(
        self,
        (card, CardSetSynopsis(syn)): (Id<Card>, &CardSetSynopsis),
    ) -> HResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        tab.insert(card, syn.as_str())?;
        Ok(())
    }
}
