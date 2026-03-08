use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DbPath(Option<PathBuf>);

impl DbPath {
    pub fn as_opt_path(&self) -> Option<&Path> {
        self.0.as_deref()
    }
}

impl Default for DbPath {
    fn default() -> Self {
        DbPath(Some(
            dirs::data_dir()
                .expect("no `dirs::data_dir` avaiable on this system")
                .join(env!("CARGO_PKG_NAME"))
                .join("db"),
        ))
    }
}

impl Display for DbPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = self.0.as_ref() {
            p.display().fmt(f)
        } else {
            ":memory:".fmt(f)
        }
    }
}

impl FromStr for DbPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == ":memory:" {
            Ok(DbPath(None))
        } else {
            s.parse().map(Some).map(DbPath)
        }
    }
}
