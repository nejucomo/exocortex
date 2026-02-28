use derive_more::{From, Into};
use derive_new::new;

/// A unique identifier scoped to a given [Provider](crate::Provider)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Into, new)]
pub struct Id(u64);

impl Id {
    /// Unwrap the u64 value
    pub fn into_u64(self) -> u64 {
        self.0
    }
}
