use std::borrow::Cow;
use std::path::PathBuf;

use aliri_braid::{Normalizer, Validator, braid};

use crate::error::InvalidPath;

const SEPARATOR: &str = " > ";
const SEPARATOR_CHAR: char = '>';

#[braid(normalizer)]
pub struct PagePath;

impl Validator for PagePath {
    type Error = InvalidPath;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        use InvalidPath::*;

        for part in raw.split(SEPARATOR_CHAR) {
            if part.is_empty() {
                return Err(EmptySegment);
            } else if part
                .strip_prefix(' ')
                .and_then(|s| s.strip_suffix(' '))
                .filter(|&s| s.trim() == s)
                .is_none()
            {
                return Err(ForbiddenWhitespace);
            }
        }
        Ok(())
    }
}

impl Normalizer for PagePath {
    fn normalize<'a>(raw: &'a str) -> Result<Cow<'a, str>, Self::Error> {
        match Self::validate(raw) {
            Ok(()) => Ok(Cow::from(raw)),
            Err(InvalidPath::ForbiddenWhitespace) => Ok(Cow::from(normalize(raw))),
            Err(e) => Err(e),
        }
    }
}

impl PagePathRef {
    pub fn to_path(&self) -> PathBuf {
        let mut pb = PathBuf::default();
        for seg in self.0.split(SEPARATOR) {
            pb.push(seg.replace('/', "\\/"));
        }
        pb.set_extension("md");
        pb
    }

    pub fn split_first(&self) -> (&str, Option<Cow<'_, PagePathRef>>) {
        if let Some((seg, suffix)) = self.0.split_once(SEPARATOR) {
            let suffix =
                Self::from_str(suffix).expect("This should always be valid due to validation.");
            (seg, Some(suffix))
        } else {
            (&self.0, None)
        }
    }
}

fn normalize(raw: &str) -> String {
    let mut s = String::with_capacity(raw.len());
    let mut optprev = None;
    for ch in raw.chars() {
        if let Some(prev) = optprev {
            match (prev, ch) {
                // Deduplicate spaces:
                (' ', ' ') => continue,
                // Ensure space before `SEPARATOR_CHAR`:
                (c, SEPARATOR_CHAR) if c != ' ' => s.push(' '),
                (SEPARATOR_CHAR, c) if c != ' ' => s.push(' '),
                _ => {}
            }
        }

        s.push(ch);
        optprev = Some(ch);
    }
    s
}
