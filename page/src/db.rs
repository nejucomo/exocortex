use std::collections::BTreeMap;

use include_dir::{Dir, include_dir};
use strum::EnumString;

use crate::Page::{self, ReadOnly, ReadWrite};
use crate::error::NonexistentPage;
use crate::{PagePath, PagePathRef};

static HELP_DOCS: &Dir<'_> = &include_dir!("$CARGO_MANIFEST_DIR/help_docs");

#[derive(Debug, Default)]
pub struct PageDb {
    pages: BTreeMap<PagePath, String>,
}

impl PageDb {
    pub fn access(&mut self, path: PagePath) -> Result<Page<'_>, NonexistentPage> {
        use PathPrefix::*;

        #[derive(EnumString)]
        #[strum(serialize_all = "kebab-case")]
        enum PathPrefix {
            Help,
            Journal,
        }

        let (prefix, _) = path.split_first();
        let p: PathPrefix = prefix.parse().map_err(|_| NonexistentPage)?;
        match p {
            Help => access_help(&path).map(ReadOnly),
            Journal => Ok(ReadWrite(self.pages.entry(path).or_default())),
        }
    }
}

fn access_help(path: &PagePathRef) -> Result<&'static str, NonexistentPage> {
    let path = path.to_path();
    // log::debug!("open_help({:?})", path.display());
    HELP_DOCS
        .get_file(path)
        .map(|f| f.contents_utf8().expect("static help doc is non-utf8"))
        .ok_or(NonexistentPage)
}
