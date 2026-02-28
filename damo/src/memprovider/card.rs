use time::OffsetDateTime;

#[derive(Debug)]
pub(super) struct MemCard {
    pub ctime: OffsetDateTime,
    pub synopsis: String,
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
