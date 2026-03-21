use std::sync::mpsc::{self, Receiver, SyncSender};

use derive_more::{From, Into};
use derive_new::new;

/// Channel sender/receiver endpoints for message `M`
#[derive(Debug, From, Into, new)]
pub struct Channel<M> {
    /// The sender endpoint
    pub sender: SyncSender<M>,
    /// The receiver endpoint
    pub receiver: Receiver<M>,
}

impl<M> Channel<M> {
    /// Allocate a new channel
    pub fn alloc(bound: usize) -> Self {
        Self::from(mpsc::sync_channel(bound))
    }
}
