use derive_more::{From, Into};
use derive_new::new;
use exocortex_redborm::{Load, OrmResult, Store};
use redb::{ReadTransaction, Value, WriteTransaction};

use crate::Timestamp;

/// A `T` value with an associated timestamp
///
/// Typically this represents a creation time for a db entity
#[derive(From, Into, new)]
pub struct Timestamped<T> {
    /// The time associated with this value
    pub time: Timestamp,
    /// The value
    pub val: T,
}

impl<T> Timestamped<T> {
    /// Associate the current time to `val`
    pub fn now(val: T) -> Self {
        let time = Timestamp::now();
        Timestamped { time, val }
    }
}

impl<T> Store for Timestamped<T>
where
    T: Store,
{
    type KOV = Timestamped<T::KOV>;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        let Timestamped { time, val } = self;
        let kov = val.store_into(txn)?;
        Ok(Timestamped::new(time, kov))
    }
}

impl<T> Load for Timestamped<T>
where
    T: Load,
{
    type KOV = Timestamped<T::KOV>;

    fn load_from(
        txn: &ReadTransaction,
        kov: <Self::KOV as Value>::SelfType<'_>,
    ) -> OrmResult<Self> {
        let (time, innerkov) = kov.into();
        let val = T::load_from(txn, innerkov)?;
        Ok(Timestamped { time, val })
    }
}

impl<T> std::fmt::Debug for Timestamped<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamped<{}: ", self.time)?;
        self.val.fmt(f)?;
        write!(f, ">")?;
        Ok(())
    }
}

impl<T> Value for Timestamped<T>
where
    T: Value,
{
    type SelfType<'a>
        = Timestamped<T::SelfType<'a>>
    where
        Self: 'a;

    type AsBytes<'a>
        = <(Timestamp, T) as Value>::AsBytes<'a>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        <(Timestamp, T) as Value>::fixed_width()
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        Timestamped::from(<(Timestamp, T) as Value>::from_bytes(data))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.into().as_bytes()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new(std::any::type_name::<Self>())
    }
}
