use std::sync::mpsc::{Receiver, SyncSender};

use derive_more::{From, Into};
use derive_new::new;

/// An interface for sending `S` messages and receiving `R` messsages, such as for
#[derive(Debug, From, Into, new)]
pub struct Interface<T, F> {
    /// The `T` [SyncSender]
    pub to: SyncSender<T>,
    /// The `F` [Receiver]
    pub from: Receiver<F>,
}
