mod dboption;

use clap::Parser;

pub use self::dboption::DbOption;

/// `exocortex`: your handy attention and cognition toolkit
#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    /// The database to use: a filesystem path for redb persistence, or `:ram:` for in-memory
    #[clap(long, default_value_t)]
    pub db: DbOption,
}
