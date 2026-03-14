//! The `exocortex` database
#![deny(missing_docs, unsafe_code)]

use std::path::Path;

/// The `exocortex` database
#[derive(Debug)]
pub struct Database {}

impl Database {
    /// Open or create a new database at the given path
    pub fn open_or_create<P>(dbpath: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let _ = dbpath;
        todo!()
    }
}
