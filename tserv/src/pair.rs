use derive_more::{From, Into};
use derive_new::new;

use crate::{Channel, Interface};

/// A pair of interfaceas with complementary send/receive messages, such as for an `A` request / to `B` response system
#[derive(Debug, From, Into, new)]
pub struct InterfacePair<A, B> {
    /// send `A`, receive `B`
    pub ab: Interface<A, B>,
    /// send `B`, receive `A`
    pub ba: Interface<B, A>,
}

impl<A, B> InterfacePair<A, B> {
    /// Allocate a new interface pair
    pub fn alloc(forward_bound: usize, reverse_bound: usize) -> Self {
        let a = Channel::alloc(forward_bound);
        let b = Channel::alloc(reverse_bound);
        let ab = Interface::new(a.sender, b.receiver);
        let ba = Interface::new(b.sender, a.receiver);
        Self::new(ab, ba)
    }
}
