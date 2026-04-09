use std::any::type_name;
use std::sync::mpsc::{Receiver, SyncSender};

use derive_more::{From, Into};
use derive_new::new;

/// An interface for sending `S` messages and receiving `R` messsages, such as for
#[derive(From, Into, new, derive_more::Debug)]
#[debug("Interface<to: {} from: {}>", type_name::<T>(), type_name::<F>())]
pub(crate) struct Interface<T, F> {
    /// The `T` [SyncSender]
    pub to: SyncSender<T>,
    /// The `F` [Receiver]
    pub from: Receiver<F>,
}
