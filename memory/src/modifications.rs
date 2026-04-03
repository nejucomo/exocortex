//! Memory modifications

use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::Id;

use crate::Thop;

#[derive(Clone, Debug, From, TryInto)]
pub enum ThopModify {
    Create(Thop),
    SetSynopsis(ThopSetSynopsis),
}

#[derive(Clone, Debug, From, new)]
pub struct ThopSetSynopsis {
    pub thop: Id<Thop>,
    pub synopsis: String,
}
