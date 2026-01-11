use derive_new::new;

use crate::PagePath;

#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct PageDbEntry<'a>(std::collections::btree_map::Entry<'a, PagePath, String>);
