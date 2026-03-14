mod dbpath;

use clap::Parser;

use self::dbpath::DbPath;

/// `exocortex`: your handy attention and cognition toolkit
#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    /// The DB path
    #[clap(long, default_value_t)]
    pub db_path: DbPath,
}
