use std::sync::mpsc::{self, Receiver, Sender};

use crate::messages::{Reply, Request};

pub(crate) type ToApp = Sender<Reply>;
pub(crate) type FromApp = Receiver<Request>;

pub(crate) type ToDb = Sender<Request>;
pub(crate) type FromDb = Receiver<Reply>;

pub(crate) fn channel_pair() -> ((ToDb, FromDb), (ToApp, FromApp)) {
    let (to_db, from_app) = mpsc::channel();
    let (to_app, from_db) = mpsc::channel();

    ((to_db, from_db), (to_app, from_app))
}
