use redb::{ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction};

use crate::dbio::{ReadTransactionLoad as _, WriteTransactionStore as _};
use crate::entities::Card;
use crate::messages::{
    CardModify, DbIsEmpty, DbReply, DbRequest, LogScan, Queried, Query, Request, ScannedItems,
};
use crate::{Id, Result, Timestamped, tables};

pub(crate) trait Handler<R: Request> {
    fn handle(&mut self, request: R) -> Result<R::Reply>;
}

struct HiddenSentinel;

impl Request for (HiddenSentinel, DbRequest) {
    type Reply = DbReply;
}

impl Handler<DbRequest> for redb::Database {
    fn handle(&mut self, request: DbRequest) -> Result<DbReply> {
        log::trace!("{:#?}", &request);
        let res = self.handle((HiddenSentinel, request));
        log::trace!("{:#?}", &res);
        res
    }
}

impl Handler<(HiddenSentinel, DbRequest)> for redb::Database {
    fn handle(&mut self, (_, request): (HiddenSentinel, DbRequest)) -> Result<DbReply> {
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

impl Handler<CardModify> for redb::Database {
    fn handle(&mut self, m: CardModify) -> Result<Id<Card>> {
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
    fn handle(&mut self, LogScan: LogScan) -> Result<ScannedItems> {
        self.scan()
    }
}

impl Handler<CardModify> for WriteTransaction {
    fn handle(&mut self, m: CardModify) -> Result<()> {
        let item = self.store(m)?;
        log::debug!("dropping {item:?} from memory.");
        Ok(())
    }
}
