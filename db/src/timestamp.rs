use std::cmp::Ordering;

use jiff::Timestamp as JTS;
use redb::{Key, Value};

/// A UTC timestamp with microsecond precision
#[derive(Copy, Clone, Debug)]
pub struct Timestamp(JTS);

impl Timestamp {
    pub fn now() -> Self {
        Self(JTS::now())
    }

    pub fn from_microseconds(t: i64) -> Self {
        Self(JTS::from_microsecond(t).unwrap())
    }

    pub fn into_microseconds(self) -> i64 {
        self.0.as_microsecond()
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
