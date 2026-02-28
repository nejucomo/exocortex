use std::borrow::Cow;

use derive_more::{From, Into};

/// A <u>C</u>ard <u>id</u>entifier
#[derive(Debug, From, Into)]
pub struct Id(u64);

/// A data model instance
pub trait Provider {
    type Id: Copy;
    type Error;

    fn new_card(&mut self) -> Result<Self::Id, Self::Error>;
    fn set_synopsis(&mut self, id: Id, s: &str) -> Result<(), Self::Error>;
    fn get_synopsis(&mut self, id: Id) -> Result<Cow<'_, str>, Self::Error>;
}
