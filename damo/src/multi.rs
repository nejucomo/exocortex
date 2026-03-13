use std::path::Path;

use enum_dispatch::enum_dispatch;

use crate::{DamoResult, MemProvider, RedProvider};

#[allow(unused_imports)] // Silence a false-positive necessary for `enum_dispatch`
use crate::Provider;

#[derive(Debug)]
#[enum_dispatch(Provider)]
pub enum MultiProvider {
    Mem(MemProvider),
    Red(RedProvider),
}

impl MultiProvider {
    pub fn open_or_create<P>(optpath: Option<P>) -> DamoResult<Self>
    where
        P: AsRef<Path>,
    {
        use MultiProvider::*;

        if let Some(p) = optpath {
            RedProvider::open_or_create(p).map(Red)
        } else {
            Ok(Mem(MemProvider::default()))
        }
    }
}
