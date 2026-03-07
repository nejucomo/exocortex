use std::path::{Path, PathBuf};

use clap::{Args, Parser};

/// `exocortex`: your handy attention and cognition toolkit
#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    #[clap(flatten)]
    pub dbopts: DbOptions,
}

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct DbOptions {
    /// Use ephemeral memory for the DB, aka no persistence
    #[clap(long)]
    db_mem: bool,

    /// Use the given database path for persistence
    #[clap(long)]
    db_path: Option<PathBuf>,
}

impl DbOptions {
    /// Get the database path; if `None` use memory only
    pub fn path(&self) -> Option<&Path> {
        if self.db_mem {
            if self.db_path.is_none() {
                None
            } else {
                unreachable!("clap arg parsing post-condition failure: {self:?}");
            }
        } else if let Some(p) = self.db_path.as_deref() {
            Some(p)
        } else {
            todo!("default db path")
        }
    }
}
