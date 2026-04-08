use derive_more::From;
use exocortex_lid::{Id, ValueWithId as _, WithId};
use exocortex_memory::modifications::{ThopModified, ThopModify, ThopMutation, ThopSetSynopsis};
use exocortex_memory::queries::{Queried, Query, ScanQueried, ScanQuery, ThopCount, ThopCounted};
use exocortex_memory::{Reply, ReplyInfo, Request, RequestInfo, Thop};
use exocortex_redborm::OrmResult;
use exocortex_redborm::enumvalue::EnumColumnar;
use exocortex_redborm::ext::WriteTransactionExt as _;
use exocortex_timestamp::Timestamp;
use redb::{Database, ReadTransaction, ReadableDatabase as _, WriteTransaction};

use crate::entities::{ThopModificationV0, ThopSetSynopsisV0, ThopV0};
use crate::{RedError, RedResult};

#[derive(Debug, From)]
pub(crate) struct MemImpl {
    redb: Database,
}

impl MemImpl {
    fn handle_top_level(&mut self, request: Request) -> RedResult<Reply> {
        let reply_info = self.handle(request.info())?;
        Ok(Reply {
            request,
            reply_info,
        })
    }
}

impl exocortex_handler::SyncHandler<Request> for MemImpl {
    type Reply = Reply;
    type SyncError = RedError;

    fn handle(&mut self, request: Request) -> RedResult<Reply> {
        log::trace!("{:#?}", &request);
        let res = self.handle_top_level(request);
        log::trace!("{:#?}", &res);
        res
    }
}

trait RedHandler<R> {
    type Reply;

    fn handle(self, req: &R) -> OrmResult<Self::Reply>;
}

impl RedHandler<RequestInfo> for &mut MemImpl {
    type Reply = ReplyInfo;

    fn handle(self, req: &RequestInfo) -> OrmResult<Self::Reply> {
        use ReplyInfo::*;
        use RequestInfo::*;

        match req {
            Query(q) => {
                let txn = self.redb.begin_read()?;
                txn.handle(q).map(Queried)
            }
            Modify(m) => {
                let txn = self.redb.begin_write()?;
                let res = txn.handle(m).map(Modified);
                if res.is_ok() {
                    txn.commit()?;
                }
                res
            }
        }
    }
}

impl RedHandler<Query> for ReadTransaction {
    type Reply = Queried;

    fn handle(self, q: &Query) -> OrmResult<Self::Reply> {
        use Queried::*;
        use Query::*;

        match q {
            ThopCount(q) => self.handle(q).map(ThopCounted),
            Scan(q) => self.handle(q).map(Scanned),
        }
    }
}

impl RedHandler<ThopCount> for ReadTransaction {
    type Reply = ThopCounted;

    fn handle(self, _: &ThopCount) -> OrmResult<Self::Reply> {
        todo!()
    }
}

impl RedHandler<ScanQuery> for ReadTransaction {
    type Reply = ScanQueried;

    fn handle(self, _: &ScanQuery) -> OrmResult<Self::Reply> {
        todo!()
    }
}

impl RedHandler<ThopModify> for &WriteTransaction {
    type Reply = WithId<ThopModified>;

    fn handle(self, req: &ThopModify) -> OrmResult<Self::Reply> {
        use ThopModify::*;

        // Time is approximate somewhere in the midst of the txn
        let time = Timestamp::now();

        match req {
            Create(v) => {
                let thop = self.handle(v)?;

                let idmod = self.store(ThopModificationV0 {
                    thop,
                    time,
                    enumcol: EnumColumnar {
                        variant: 0,
                        referent: thop.transmute(),
                    },
                })?;

                Ok(ThopModified {
                    thop,
                    time,
                    info: ThopMutation::Created,
                }
                .with_id(idmod.transmute()))
            }
            SetSynopsis(v) => {
                let thop = v.thop;
                let syn = v.synopsis.clone(); // TODO: Can we remove string copy?
                let idtss = self.handle(v)?;

                let idmod = self.store(ThopModificationV0 {
                    thop,
                    time,
                    enumcol: EnumColumnar {
                        variant: 1,
                        referent: idtss.transmute(),
                    },
                })?;

                Ok(ThopModified {
                    thop,
                    time,
                    info: ThopMutation::SetSynopsis(syn),
                }
                .with_id(idmod.transmute()))
            }
        }
    }
}

impl RedHandler<Thop> for &WriteTransaction {
    type Reply = Id<Thop>;

    fn handle(self, _: &Thop) -> OrmResult<Self::Reply> {
        self.store(ThopV0).map(Id::transmute::<Thop>)
    }
}

impl RedHandler<ThopSetSynopsis> for &WriteTransaction {
    type Reply = Id<ThopSetSynopsisV0>;

    fn handle(self, tss: &ThopSetSynopsis) -> OrmResult<Self::Reply> {
        self.store(ThopSetSynopsisV0::from(tss))
    }
}
