use derive_more::{Deref, DerefMut};
use derive_new::new;
use exocortex_redb::messages::{CardScan, Request};
use exocortex_redb::{ExoDb, Id};

#[derive(Debug, new, Deref, DerefMut)]
pub(crate) struct DbManager {
    #[deref]
    #[deref_mut]
    db: ExoDb,
    #[new(default)]
    outstanding_scan_request: Option<Id<Request>>,
}

impl DbManager {
    pub(crate) fn post_scan_request_if_none_outstanding(&mut self, req: CardScan) {
        self.outstanding_scan_request
            .get_or_insert_with(|| self.db.post_request(req));
    }
}
