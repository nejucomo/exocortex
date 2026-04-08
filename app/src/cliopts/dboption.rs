use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

/// Which database backend to use
#[derive(Clone, Debug)]
pub enum DbOption {
    /// Use an in-memory RAM provider (no persistence), selected with `:ram:`
    Ram,
    /// Use a redb-backed persistent database at the given filesystem path
    Path(PathBuf),
}

impl Default for DbOption {
    fn default() -> Self {
        DbOption::Path(
            dirs::data_dir()
                .expect("no `dirs::data_dir` available on this system")
                .join(env!("CARGO_PKG_NAME"))
                .join("db"),
        )
    }
}

impl Display for DbOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbOption::Ram => write!(f, ":ram:"),
            DbOption::Path(p) => p.display().fmt(f),
        }
    }
}

impl FromStr for DbOption {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == ":ram:" {
            Ok(DbOption::Ram)
        } else {
            Ok(DbOption::Path(PathBuf::from(s)))
        }
    }
}
