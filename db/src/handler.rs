use redb::{ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction};

use crate::entities::Card;
use crate::messages::{DbIsEmpty, DbReply, DbRequest, LogScan, Modify, Queried, Query, Request};
use crate::store::WriteTransactionStore as _;
use crate::{Id, Result, tables};

pub(crate) trait Handler<R: Request> {
    fn handle(&mut self, request: R) -> Result<R::Reply>;
}

impl Handler<DbRequest> for redb::Database {
    fn handle(&mut self, request: DbRequest) -> Result<DbReply> {
        use DbReply::*;
        use DbRequest::*;

        match request {
            Query(q) => self.handle(q).map(Queried),
            Modify(m) => self.handle(m).map(Modified),
        }
    }
}

impl Handler<Query> for redb::Database {
    fn handle(&mut self, q: Query) -> Result<Queried> {
        let mut txn = self.begin_read()?;
        txn.handle(q)
    }
}

impl Handler<Modify> for redb::Database {
    fn handle(&mut self, m: Modify) -> Result<Id<Card>> {
        let mut txn = self.begin_write()?;
        let reply = txn.handle(m)?;
        txn.commit()?;
        Ok(reply)
    }
}

impl Handler<Query> for ReadTransaction {
    fn handle(&mut self, q: Query) -> Result<Queried> {
        use Queried::*;
        use Query::*;

        match q {
            DbIsEmpty(x) => self.handle(x).map(DbWasEmpty),
            LogScan(x) => self.handle(x).map(LogScanned),
        }
    }
}

impl Handler<DbIsEmpty> for ReadTransaction {
    fn handle(&mut self, _: DbIsEmpty) -> Result<bool> {
        use redb::TableError::TableDoesNotExist;

        match self.open_table(tables::CARD_CREATE_V0) {
            Ok(tab) => {
                let len = tab.len()?;
                Ok(len == 0)
            }
            Err(TableDoesNotExist(_)) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }
}

impl Handler<LogScan> for ReadTransaction {
    fn handle(&mut self, request: LogScan) -> Result<Vec<Modify>> {
        let _ = request;
        todo!()
    }
}

impl Handler<Modify> for WriteTransaction {
    fn handle(&mut self, m: Modify) -> Result<Id<Card>> {
        let (modid, card) = self.store(&m)?;
        log::debug!("dropping {modid:?} from memory.");
        Ok(card)
    }
}
