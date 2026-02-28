use exocortex_damo::{Card, ProviderErrors};
use time::OffsetDateTime;

use crate::UnknownId;

/// An in-memory card representation
#[derive(Debug)]
pub struct MemCard {
    ctime: OffsetDateTime,
    synopsis: String,
}

impl MemCard {
    pub(super) fn new() -> Self {
        use OffsetDateTime as ODT;

        MemCard {
            ctime: ODT::now_local().unwrap_or_else(|_| ODT::now_utc()),
            synopsis: "".to_string(),
        }
    }
}

impl ProviderErrors for MemCard {
    type UpdateError = UnknownId;
    type QueryError = UnknownId;
}

impl Card for MemCard {
    fn get_time_of_creation(&self) -> Result<OffsetDateTime, Self::QueryError> {
        Ok(self.ctime)
    }

    fn get_synopsis(&self) -> Result<&str, Self::QueryError> {
        Ok(self.synopsis.as_str())
    }

    fn set_synopsis(&mut self, synopsis: &str) -> Result<(), Self::UpdateError> {
        self.synopsis = synopsis.to_string();
        Ok(())
    }
}
