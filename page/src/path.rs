use std::path::PathBuf;

use aliri_braid::{Validator, braid};

use crate::error::{InvalidPath, PageError};

#[braid(validator)]
pub struct PagePath;

impl PagePath {
    const SEPARATOR: &str = ">";
}

impl Validator for PagePath {
    type Error = PageError;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        use InvalidPath::*;

        for part in raw.split(Self::SEPARATOR) {
            if part.is_empty() {
                return Err(PageError::new(raw, EmptySegment));
            } else if part.trim() != part {
                return Err(PageError::new(raw, ForbiddenWhitespace));
            }
        }
        Ok(())
    }
}

impl PagePathRef {
    pub fn to_path(&self) -> PathBuf {
        let mut pb = PathBuf::default();
        for seg in self.0.split(PagePath::SEPARATOR) {
            pb.push(seg.replace('/', "\\/"));
        }
        pb.set_extension("md");
        pb
    }

    pub fn split_first(&self) -> (&str, Option<&PagePathRef>) {
        if let Some((seg, suffix)) = self.0.split_once(PagePath::SEPARATOR) {
            let suffix =
                Self::from_str(suffix).expect("This should always be valid due to validation.");
            (seg, Some(suffix))
        } else {
            (&self.0, None)
        }
    }
}
