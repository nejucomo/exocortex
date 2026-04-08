use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::{Id, WithId};

use crate::modifications::ThopModified;

#[derive(Copy, Clone, Debug)]
pub struct Scan;

#[derive(Copy, Clone, Debug, From, TryInto)]
pub enum ScanQuery {
    Start(Scan),
    Advance(ScanNext),
    Release(ScanRelease),
}

#[derive(Copy, Clone, Debug, From, new)]
pub struct ScanNext(pub Id<Scan>);

#[derive(Copy, Clone, Debug, From, new)]
pub struct ScanRelease(pub Id<Scan>);

#[derive(Debug, From, TryInto)]
pub enum ScanQueried {
    Started(Id<Scan>),
    Advanced(WithId<ThopModified>),
    Released(ScanReleased),
}

#[derive(Copy, Clone, Debug)]
pub struct ScanReleased;
