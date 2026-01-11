use include_dir::{Dir, include_dir};

use crate::error::PageError;
use crate::error::PageErrorReason::{self, MalformedUtf8, Nonexistent};
use crate::path::PagePathRef;

static HELP_DOCS: &Dir<'_> = &include_dir!("$CARGO_MANIFEST_DIR/help_docs");

#[derive(Debug)]
pub enum Page {
    Static(&'static str),
}

impl Page {
    pub fn open_path<P>(path: P) -> Result<Self, PageError>
    where
        P: AsRef<PagePathRef>,
    {
        let ppref = path.as_ref();
        log::debug!("Page::open_path({ppref:?})");
        Self::open_path_reason(ppref).map_err(|reason| PageError::new(ppref.to_string(), reason))
    }

    fn open_path_reason(path: &PagePathRef) -> Result<Self, PageErrorReason> {
        let (seg, _optsuffix) = path.split_first();
        if seg == "help" {
            Self::open_help_path(path)
        } else {
            Err(Nonexistent)
        }
    }

    fn open_help_path(path: &PagePathRef) -> Result<Self, PageErrorReason> {
        let path = path.to_path();
        log::debug!("Opening help: {:?}", path.display());
        let f = HELP_DOCS.get_file(path).ok_or(Nonexistent)?;
        let text = f.contents_utf8().ok_or(MalformedUtf8)?;
        Ok(Page::Static(text))
    }
}
