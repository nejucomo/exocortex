use std::sync::Arc;
use std::thread::JoinHandle;

use logself::LogSelf as _;
use redb::{
    Database, ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction,
};

use crate::channel::{FromApp, ToApp};
use crate::messages::{
    Card, CardCreate, CardModification, CardModify, CardScan, CardScanned, CardSetSynopsis,
    CardUpdated, DbIsEmpty, Modify, Queried, Query, RepSpec, Reply, ReqSpec, Request,
};
use crate::tables::TABLES;
use crate::{DbResult, Id};

pub(crate) fn launch(db: Database, to_from_app: (ToApp, FromApp)) -> JoinHandle<DbResult<()>> {
    std::thread::Builder::new()
        .name(env!("CARGO_PKG_NAME").to_string())
        .spawn(|| run_db_thread(db, to_from_app))
        .unwrap()
}

fn run_db_thread(db: Database, to_from_app: (ToApp, FromApp)) -> DbResult<()> {
    run_inner(db, to_from_app).inspect_err(|e| {
        e.log_warn_ref("db thread error");
    })
}

fn run_inner(mut db: Database, (to_app, from_app): (ToApp, FromApp)) -> DbResult<()> {
    let mut scan: Option<Scan> = None;

    log::debug!("db request-handler loop starting");
    // The `RecvError` is a unit-type; no info is lost by dropping it:
    while let Ok(req) = from_app.recv() {
        log::debug!("processing {:?}: {:?}", req.id, &req.reqspec);
        let rep = (&mut db, &mut scan).handle(req)?;
        log::debug!("sending response: {:?}", &rep);
        to_app.send(rep)?;
    }
    log::debug!("db request-handler loop exiting cleanly");
    Ok(())
}

type Scan = ();

type WithScan<'a, T> = (T, &'a mut Option<Scan>);

trait Handler<R> {
    type Reply;

    fn handle(self, request: R) -> DbResult<Self::Reply>;
}

impl Handler<Request> for WithScan<'_, &mut Database> {
    type Reply = Reply;

    fn handle(self, Request { id, reqspec }: Request) -> DbResult<Self::Reply> {
        let repspec = self.handle(reqspec)?;

        Ok(Reply { reqid: id, repspec })
    }
}

impl Handler<ReqSpec> for WithScan<'_, &mut Database> {
    type Reply = RepSpec;

    fn handle(self, request: ReqSpec) -> DbResult<Self::Reply> {
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

    fn handle(self, q: Query) -> DbResult<Self::Reply> {
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

    fn handle(self, _: DbIsEmpty) -> DbResult<Self::Reply> {
        use redb::TableError::TableDoesNotExist;

        match self.open_table(TABLES.card_synopsis) {
            Ok(tab) => {
                let len = tab.len()?;
                Ok(len == 0)
            }
            Err(TableDoesNotExist(_)) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }
}

impl Handler<CardScan> for WithScan<'_, ReadTransaction> {
    type Reply = CardScanned;

    fn handle(self, request: CardScan) -> DbResult<Self::Reply> {
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

    fn handle(self, action: Modify) -> DbResult<Self::Reply> {
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

    fn handle(self, _: CardCreate) -> DbResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        let card = tab.len().map(Id::new)?;
        tab.insert(card, "")?;
        Ok(card)
    }
}

impl Handler<Arc<CardModify>> for WriteTransaction {
    type Reply = Arc<CardModify>;

    fn handle(self, acm: Arc<CardModify>) -> DbResult<Self::Reply> {
        let CardModify { card, modif } = acm.as_ref();
        self.handle((*card, modif))?;
        Ok(acm)
    }
}

impl Handler<(Id<Card>, &CardModification)> for WriteTransaction {
    type Reply = ();

    fn handle(self, (card, cmod): (Id<Card>, &CardModification)) -> DbResult<Self::Reply> {
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
    ) -> DbResult<Self::Reply> {
        let mut tab = self.open_table(TABLES.card_synopsis)?;
        tab.insert(card, syn.as_str())?;
        Ok(())
    }
}
