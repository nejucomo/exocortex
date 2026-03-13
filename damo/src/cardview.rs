use derive_new::new;

use crate::Id;

#[derive(Debug, new)]
pub struct CardView<'a> {
    pub id: Id,
    pub synopsis: &'a str,
}
