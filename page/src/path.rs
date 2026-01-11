use std::borrow::Cow;
use std::path::PathBuf;

use aliri_braid::{Normalizer, Validator, braid};

use crate::error::InvalidPath;

const SEPARATOR: &str = " > ";
const SEPARATOR_CHAR: char = '>';

#[braid(normalizer)]
pub struct PagePath;

impl Default for PagePath {
    fn default() -> Self {
        Self::from_static("help > welcome")
    }
}

impl Validator for PagePath {
    type Error = InvalidPath;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        use InvalidPath::*;

        for part in raw.split(SEPARATOR) {
            if part.is_empty() {
                return Err(EmptySegment);
            } else if part.trim() != part {
                return Err(ForbiddenWhitespace);
            } else if part.find(SEPARATOR_CHAR).is_some() {
                return Err(MissingWhitespace);
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

impl PagePath {
    pub fn try_push<S>(&mut self, seg: S) -> Result<(), InvalidPath>
    where
        S: AsRef<str>,
    {
        let normcow = Self::normalize(seg.as_ref())?;
        self.0.push_str(SEPARATOR);
        self.0.push_str(&normcow);
        Ok(())
    }

    pub fn push<S>(&mut self, seg: S)
    where
        S: AsRef<str>,
    {
        self.try_push(seg).unwrap();
    }

    pub fn join<S>(mut self, seg: S) -> Self
    where
        S: AsRef<str>,
    {
        self.push(seg);
        self
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

    pub fn split_first(&self) -> (&str, Option<&PagePathRef>) {
        if let Some((seg, suffix)) = self.0.split_once(SEPARATOR) {
            let suffix = Self::from_normalized_str(suffix).expect("This should always be valid");
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
