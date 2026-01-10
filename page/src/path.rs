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
        pb
    }

    pub fn split_first(&self) -> (&str, Option<&PagePathRef>) {
        let (seg, suffix) = self
            .0
            .split_once(PagePath::SEPARATOR)
            .expect("There should always be at least one segment due to validation.");

        let subseq = if suffix.is_empty() {
            None
        } else {
            Some(Self::from_str(suffix).expect("This should always be valid due to validation."))
        };

        (seg, subseq)
    }
}
