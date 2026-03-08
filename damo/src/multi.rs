use std::path::Path;

use canopydb::Error;
use enum_dispatch::enum_dispatch;

use crate::{CanopyProvider, MemProvider};

#[allow(unused_imports)] // Silence a false-positive necessary for `enum_dispatch`
use crate::Provider;

#[derive(Debug)]
#[enum_dispatch(Provider)]
pub enum MultiProvider {
    Mem(MemProvider),
    Canopy(CanopyProvider),
}

impl MultiProvider {
    pub fn open_or_create<P>(optpath: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        use MultiProvider::*;

        if let Some(p) = optpath {
            CanopyProvider::open_or_create(p).map(Canopy)
        } else {
            Ok(Mem(MemProvider::default()))
        }
    }
}
