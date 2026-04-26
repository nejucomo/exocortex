use std::fmt::Display;

use jiff::Zoned;
use jiff::tz::TimeZone;

/// A localized timestamp
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timestamp(Zoned);

impl Timestamp {
    /// The time now
    pub fn now() -> Self {
        Self(Zoned::now())
    }

    /// The given a number of microseconds since the unix epoch as a [Timestamp]
    pub fn from_microseconds(t: i64) -> Self {
        Self(
            jiff::Timestamp::from_microsecond(t)
                .unwrap()
                .to_zoned(TimeZone::system()),
        )
    }

    /// `self` as a number of microseconds since the unix epoch
    pub fn microseconds(&self) -> i64 {
        self.0.timestamp().as_microsecond()
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "redb")]
impl redb::Value for Timestamp {
    type SelfType<'a>
        = Timestamp
    where
        Self: 'a;

    type AsBytes<'a>
        = String
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let text: &str = str::from_utf8(data).expect("non-utf8 `Timestamp` deserialization");
        Timestamp(text.parse().unwrap())
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.0.to_string()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new(std::any::type_name::<Timestamp>())
    }
}

#[cfg(feature = "redb")]
impl redb::Key for Timestamp {
    fn compare(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        a.cmp(b)
    }
}
