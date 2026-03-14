use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use derive_more::{AsRef, Deref};

#[derive(Clone, Debug, Deref, AsRef)]
#[as_ref(forward)]
pub struct DbPath(PathBuf);

impl Default for DbPath {
    fn default() -> Self {
        DbPath(
            dirs::data_dir()
                .expect("no `dirs::data_dir` avaiable on this system")
                .join(env!("CARGO_PKG_NAME"))
                .join("db"),
        )
    }
}

impl Display for DbPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl FromStr for DbPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(DbPath)
    }
}
