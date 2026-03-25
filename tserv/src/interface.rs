use std::sync::mpsc::{Receiver, SyncSender};

use derive_more::{From, Into};
use derive_new::new;

/// An interface for sending `S` messages and receiving `R` messsages, such as for
#[derive(From, Into, new)]
pub(crate) struct Interface<T, F> {
    /// The `T` [SyncSender]
    pub to: SyncSender<T>,
    /// The `F` [Receiver]
    pub from: Receiver<F>,
}

impl<T, F> std::fmt::Debug for Interface<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interface<to: {} from: {}>",
            std::any::type_name::<T>(),
            std::any::type_name::<F>()
        )
    }
}
