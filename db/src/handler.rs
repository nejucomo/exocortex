use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_redborm::{OrmResult, RowValue as _};
use redb::{ReadTransaction, ReadableDatabase as _, ReadableTableMetadata as _, WriteTransaction};

use crate::BlurbId;
use crate::entities::BlurbV0;
use crate::messages::{
    BlurbModify, DbIsEmpty, DbReply, DbRequest, LogScan, LogScanItems, Queried, Query, Request,
};

pub(crate) trait Handler<R: Request> {
    fn handle(&mut self, request: R) -> OrmResult<R::Reply>;
}

struct HiddenSentinel;

impl Request for (HiddenSentinel, DbRequest) {
    type Reply = DbReply;
}

impl Handler<DbRequest> for redb::Database {
    fn handle(&mut self, request: DbRequest) -> OrmResult<DbReply> {
        log::trace!("{:#?}", &request);
        let res = self.handle((HiddenSentinel, request));
        log::trace!("{:#?}", &res);
        res
    }
}

impl Handler<(HiddenSentinel, DbRequest)> for redb::Database {
    fn handle(&mut self, (_, request): (HiddenSentinel, DbRequest)) -> OrmResult<DbReply> {
        use DbReply::*;
        use DbRequest::*;

        match request {
            Query(q) => self.handle(q).map(Queried),
            Modify(m) => self.handle(m).map(Modified),
        }
    }
}

impl Handler<Query> for redb::Database {
    fn handle(&mut self, q: Query) -> OrmResult<Queried> {
        let mut txn = self.begin_read()?;
        txn.handle(q)
    }
}

impl Handler<BlurbModify> for redb::Database {
    fn handle(&mut self, m: BlurbModify) -> OrmResult<BlurbId> {
        let mut txn = self.begin_write()?;
        let id = txn.handle(m)?;
        txn.commit()?;
        Ok(id)
    }
}

impl Handler<Query> for ReadTransaction {
    fn handle(&mut self, q: Query) -> OrmResult<Queried> {
        use Queried::*;
        use Query::*;

        match q {
            DbIsEmpty(x) => self.handle(x).map(DbWasEmpty),
            LogScan(x) => self.handle(x).map(LogScanned),
        }
    }
}

impl Handler<DbIsEmpty> for ReadTransaction {
    fn handle(&mut self, _: DbIsEmpty) -> OrmResult<bool> {
        use redb::TableError::TableDoesNotExist;

        match self.open_table(BlurbV0::table_definition()) {
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
    fn handle(&mut self, LogScan: LogScan) -> OrmResult<LogScanItems> {
        let mut items = vec![];
        self.scan(|k, v| {
            items.push((k, v));
            Ok(())
        })?;
        Ok(items)
    }
}

impl Handler<BlurbModify> for WriteTransaction {
    fn handle(&mut self, m: BlurbModify) -> OrmResult<BlurbId> {
        log::trace!("recording: {:?}", &m);
        let id = self.store(m)?;
        log::trace!("recorded: {id:?}");
        Ok(id)
    }
}
