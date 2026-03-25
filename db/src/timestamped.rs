use crate::Timestamp;

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
