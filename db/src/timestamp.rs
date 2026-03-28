use std::cmp::Ordering;
use std::fmt::Display;

use jiff::tz::TimeZone;
use jiff::{Timestamp as JTS, Zoned};
use redb::{Key, Value};

use crate::Timestamped;

/// A UTC timestamp with microsecond precision
#[derive(Copy, Clone, Debug)]
pub struct Timestamp(JTS);

impl Timestamp {
    /// The time now
    pub fn now() -> Self {
        Self(JTS::now())
    }

    /// The given a number of microseconds since the unix epoch as a [Timestamp]
    pub fn from_microseconds(t: i64) -> Self {
        Self(JTS::from_microsecond(t).unwrap())
    }

    /// `self` as a number of microseconds since the unix epoch
    pub fn into_microseconds(self) -> i64 {
        self.0.as_microsecond()
    }

    /// Stamp a time onto `val`
    pub fn stamp<T>(self, val: T) -> Timestamped<T> {
        Timestamped::new(self, val)
    }

    fn zoned_local(self) -> Zoned {
        self.0.to_zoned(TimeZone::system())
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.zoned_local().fmt(f)
    }
}

impl Value for Timestamp {
    type SelfType<'a>
        = Timestamp
    where
        Self: 'a;
    type AsBytes<'a>
        = [u8; 8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(8)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let bytes: [u8; 8] = data.try_into().expect("Timestamp must be 8 bytes");
        Timestamp::from_microseconds(i64::from_be_bytes(bytes))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.into_microseconds().to_be_bytes()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new(std::any::type_name::<Timestamp>())
    }
}

impl Key for Timestamp {
    fn compare(a: &[u8], b: &[u8]) -> Ordering {
        a.cmp(b)
    }
}
