use std::collections::{HashMap, VecDeque};

use exocortex_handler::PollHandler;
use exocortex_lid::{Id, ValueWithId as _, WithId};
use exocortex_memory::modifications::{ThopModified, ThopModify, ThopMutation, ThopSetSynopsis};
use exocortex_memory::queries::{
    Queried, Query, Scan, ScanNext, ScanQueried, ScanQuery, ScanRelease, ScanReleased, ThopCounted,
};
use exocortex_memory::{Provider, Reply, ReplyInfo, Request, RequestInfo};
use exocortex_thop::Thop;
use exocortex_timestamp::Timestamp;

use crate::{RamError, RamResult};

/// An in-memory (non-persistent) [`Provider`]
#[derive(Debug, Default)]
pub struct RamMem {
    // next ID to assign to a new thop
    thop_count: u64,
    // next ID to assign to a new modification record
    mod_count: u64,
    // all modifications in insertion order, stored as (id_num, ThopModified)
    modifications: Vec<(u64, ThopModified)>,
    // active scan sessions indexed by scan id number
    scans: HashMap<u64, VecDeque<WithId<ThopModified>>>,
    // next ID to assign to a new scan session
    next_scan_id: u64,
    // the reply from the most recent request, waiting to be polled
    pending_reply: Option<Reply>,
}

impl RamMem {
    /// Create a new empty in-memory provider
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_request(&mut self, request: Request) -> RamResult<Reply> {
        let reply_info = self.dispatch(request.info())?;
        Ok(Reply {
            request,
            reply_info,
        })
    }

    fn dispatch(&mut self, req: &RequestInfo) -> RamResult<ReplyInfo> {
        match req {
            RequestInfo::Query(q) => self.handle_query(q).map(ReplyInfo::Queried),
            RequestInfo::Modify(m) => self.handle_modify(m).map(ReplyInfo::Modified),
        }
    }

    fn handle_query(&mut self, q: &Query) -> RamResult<Queried> {
        match q {
            Query::ThopCount(_) => {
                let count = self
                    .modifications
                    .iter()
                    .filter(|(_, m)| matches!(m.info, ThopMutation::Created))
                    .count() as u64;
                Ok(Queried::ThopCounted(ThopCounted(count)))
            }
            Query::Scan(sq) => self.handle_scan_query(sq).map(Queried::Scanned),
        }
    }

    fn handle_scan_query(&mut self, sq: &ScanQuery) -> RamResult<ScanQueried> {
        match sq {
            ScanQuery::Start(_) => {
                let scan_id_num = self.next_scan_id;
                self.next_scan_id += 1;
                let scan_id: Id<Scan> = Id::from(scan_id_num);
                let items: VecDeque<_> = self
                    .modifications
                    .iter()
                    .map(|(id_num, m)| {
                        ThopModified {
                            thop: m.thop,
                            time: m.time.clone(),
                            info: m.info.clone(),
                        }
                        .with_id(Id::from(*id_num))
                    })
                    .collect();
                self.scans.insert(scan_id_num, items);
                Ok(ScanQueried::Started(scan_id))
            }
            ScanQuery::Advance(ScanNext(scan_id)) => {
                let scan = self
                    .scans
                    .get_mut(&scan_id.unwrap())
                    .unwrap_or_else(|| panic!("unknown scan id: {scan_id:?}"));
                if let Some(item) = scan.pop_front() {
                    Ok(ScanQueried::Advanced(item))
                } else {
                    self.scans.remove(&scan_id.unwrap());
                    Ok(ScanQueried::Released(ScanReleased))
                }
            }
            ScanQuery::Release(ScanRelease(scan_id)) => {
                self.scans.remove(&scan_id.unwrap());
                Ok(ScanQueried::Released(ScanReleased))
            }
        }
    }

    fn handle_modify(&mut self, req: &ThopModify) -> RamResult<WithId<ThopModified>> {
        let time = Timestamp::now();
        let mod_id_num = self.mod_count;
        self.mod_count += 1;

        let thop_modified = match req {
            ThopModify::Create(_) => {
                let thop_id: Id<Thop> = Id::from(self.thop_count);
                self.thop_count += 1;
                ThopModified {
                    thop: thop_id,
                    time,
                    info: ThopMutation::Created,
                }
            }
            ThopModify::SetSynopsis(ThopSetSynopsis { thop, synopsis }) => ThopModified {
                thop: *thop,
                time,
                info: ThopMutation::SetSynopsis(synopsis.clone()),
            },
        };

        self.modifications.push((mod_id_num, thop_modified.clone()));

        Ok(thop_modified.with_id(Id::from(mod_id_num)))
    }
}

impl Provider for RamMem {
    type Error = RamError;
}

impl PollHandler<Request> for RamMem {
    type Reply = Reply;
    type PollError = RamError;

    fn post_request(&mut self, request: Request) -> RamResult<()> {
        let reply = self.handle_request(request)?;
        self.pending_reply = Some(reply);
        Ok(())
    }

    fn poll_reply(&mut self) -> RamResult<Option<Reply>> {
        Ok(self.pending_reply.take())
    }

    fn wait_reply(&mut self) -> RamResult<Reply> {
        Ok(self
            .pending_reply
            .take()
            .expect("wait_reply called with no pending reply"))
    }
}
