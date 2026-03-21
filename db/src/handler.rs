use redb::{ReadTransaction, ReadableDatabase as _};

use crate::Result;
use crate::messages::{DbIsEmpty, DbReply, DbRequest, Queried, Query, Request};

pub(crate) trait Handler<R: Request> {
    fn handle(&mut self, request: R) -> Result<R::Reply>;
}

impl Handler<DbRequest> for redb::Database {
    fn handle(&mut self, request: DbRequest) -> Result<DbReply> {
        use DbReply::*;
        use DbRequest::*;

        match request {
            Query(q) => {
                let txn = self.begin_read()?;
                txn.handle(q).map(Queried)
            }

            Modify(m) => {
                let txn = self.begin_write()?;
                let reply = txn.handle(m).map(Modified)?;
                txn.commit()?;
                Ok(reply)
            }
        }
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
