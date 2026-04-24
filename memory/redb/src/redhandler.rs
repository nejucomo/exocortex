use std::collections::{HashMap, VecDeque};

use exocortex_lid::{Id, ValueWithId as _, WithId};
use exocortex_memory::modifications::{
    ThopCreate, ThopModified, ThopModify, ThopMutation, ThopSetSynopsis,
};
use exocortex_memory::queries::{
    Queried, Query, Scan, ScanNext, ScanQueried, ScanQuery, ScanRelease, ScanReleased, ThopCounted,
};
use exocortex_memory::{Reply, ReplyInfo, Request, RequestInfo};
use exocortex_redborm::Load as _;
use exocortex_redborm::OrmResult;
use exocortex_redborm::RowValue as _;
use exocortex_redborm::enumvalue::EnumColumnar;
use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_thop::Thop;
use exocortex_timestamp::Timestamp;
use redb::{Database, ReadableDatabase as _, WriteTransaction};

use crate::entities::{ThopModificationV0, ThopSetSynopsisV0, ThopV0};
use crate::{RedError, RedResult};

struct ScanState {
    items: VecDeque<WithId<ThopModified>>,
}

pub(crate) struct MemImpl {
    redb: Database,
    scans: HashMap<u64, ScanState>,
    next_scan_id: u64,
}

impl std::fmt::Debug for MemImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemImpl")
            .field("redb", &self.redb)
            .finish_non_exhaustive()
    }
}

impl From<Database> for MemImpl {
    fn from(redb: Database) -> Self {
        Self {
            redb,
            scans: HashMap::new(),
            next_scan_id: 0,
        }
    }
}

impl MemImpl {
    fn handle_top_level(&mut self, request: Request) -> RedResult<Reply> {
        let reply_info = self.dispatch(request.info())?;
        Ok(Reply {
            request,
            reply_info,
        })
    }

    fn dispatch(&mut self, req: &RequestInfo) -> RedResult<ReplyInfo> {
        use RequestInfo::*;

        match req {
            Query(q) => self.handle_query(q).map(ReplyInfo::Queried),
            Modify(m) => {
                let txn = self.redb.begin_write()?;
                let modified = (&txn).handle(m).map_err(RedError::from)?;
                txn.commit()?;
                Ok(ReplyInfo::Modified(modified))
            }
        }
    }

    fn handle_query(&mut self, q: &Query) -> RedResult<Queried> {
        match q {
            Query::ThopCount(_) => {
                let txn = self.redb.begin_read()?;
                let count = {
                    use redb::ReadableTableMetadata as _;
                    let tab = txn.open_table(ThopV0::table_definition())?;
                    tab.len()?
                };
                Ok(Queried::ThopCounted(ThopCounted(count)))
            }
            Query::Scan(sq) => self.handle_scan_query(sq).map(Queried::Scanned),
        }
    }

    fn handle_scan_query(&mut self, sq: &ScanQuery) -> RedResult<ScanQueried> {
        match sq {
            ScanQuery::Start(_) => self.start_scan(),
            ScanQuery::Advance(ScanNext(scan_id)) => self.advance_scan(*scan_id),
            ScanQuery::Release(ScanRelease(scan_id)) => self.release_scan(*scan_id),
        }
    }

    fn start_scan(&mut self) -> RedResult<ScanQueried> {
        let scan_id_num = self.next_scan_id;
        self.next_scan_id += 1;
        let scan_id: Id<Scan> = Id::from(scan_id_num);

        let txn = self.redb.begin_read()?;
        let mut items = VecDeque::new();

        txn.scan::<ThopModificationV0, _>(|key, modif| {
            let mutation = match modif.enumcol.variant {
                0 => ThopMutation::Created,
                1 => {
                    let tss_id: Id<ThopSetSynopsisV0> = modif.enumcol.referent.transmute();
                    let tss = ThopSetSynopsisV0::load_from(&txn, tss_id)?;
                    ThopMutation::SetSynopsis(tss.synopsis)
                }
                v => panic!(
                    "unknown ThopModification variant: {v}, expected 0 (Created) or 1 (SetSynopsis)"
                ),
            };
            let thop_modified = ThopModified {
                thop: modif.thop,
                time: modif.time,
                info: mutation,
            }
            .with_id(key.transmute());
            items.push_back(thop_modified);
            Ok(())
        })?;

        self.scans.insert(scan_id_num, ScanState { items });
        Ok(ScanQueried::Started(scan_id))
    }

    fn advance_scan(&mut self, scan_id: Id<Scan>) -> RedResult<ScanQueried> {
        let scan = self
            .scans
            .get_mut(&scan_id.unwrap())
            .unwrap_or_else(|| panic!("unknown scan id: {scan_id:?}"));
        if let Some(item) = scan.items.pop_front() {
            Ok(ScanQueried::Advanced(item))
        } else {
            self.scans.remove(&scan_id.unwrap());
            Ok(ScanQueried::Released(ScanReleased))
        }
    }

    fn release_scan(&mut self, scan_id: Id<Scan>) -> RedResult<ScanQueried> {
        self.scans.remove(&scan_id.unwrap());
        Ok(ScanQueried::Released(ScanReleased))
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

impl RedHandler<ThopCreate> for &WriteTransaction {
    type Reply = Id<Thop>;

    fn handle(self, _: &ThopCreate) -> OrmResult<Self::Reply> {
        self.store(ThopV0).map(Id::transmute::<Thop>)
    }
}

impl RedHandler<ThopSetSynopsis> for &WriteTransaction {
    type Reply = Id<ThopSetSynopsisV0>;

    fn handle(self, tss: &ThopSetSynopsis) -> OrmResult<Self::Reply> {
        self.store(ThopSetSynopsisV0::from(tss))
    }
}
