use crate::dbio::{LoadColumnar, StoreColumnar};
use crate::{Result, Timestamp};

/// A `T` value with an associated timestamp
///
/// Typically this represents a creation time for a db entity
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

impl<T> StoreColumnar for Timestamped<T>
where
    T: StoreColumnar,
{
    type RedValStore = (Timestamp, T::RedValStore);

    fn store_columnar(
        self,
        txn: &redb::WriteTransaction,
    ) -> Result<<Self::RedValStore as redb::Value>::SelfType<'static>> {
        let inner = self.val.store_columnar(txn)?;
        Ok((self.time, inner))
    }
}

impl<T> LoadColumnar for Timestamped<T>
where
    T: LoadColumnar,
{
    type RedValLoad = (Timestamp, T::RedValLoad);

    fn load_columnar<'a>(
        txn: &redb::ReadTransaction,
        v: <<Self as LoadColumnar>::RedValLoad as redb::Value>::SelfType<'a>,
    ) -> Result<Self> {
        let (time, inner) = v;
        let val = T::load_columnar(txn, inner)?;
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
