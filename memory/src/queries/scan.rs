use derive_more::{From, TryInto};
use derive_new::new;

use crate::Id;
use crate::modifications::ThopModify;

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

#[derive(Clone, Debug, From, TryInto)]
pub enum ScanQueried {
    Started(Id<Scan>),
    Advanced(ThopModify),
    Released(ScanReleased),
}

#[derive(Copy, Clone, Debug)]
pub struct ScanReleased;
