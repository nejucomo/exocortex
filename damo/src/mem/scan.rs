use std::collections::btree_map;

use derive_new::new;

use crate::mem::MemCard;
use crate::{CardView, DamoResult, Id};

#[derive(Debug, new)]
#[new(visibility = "pub(super)")]
pub struct CardScan<'a>(btree_map::Iter<'a, Id, MemCard>);

impl<'a> Iterator for CardScan<'a> {
    type Item = DamoResult<CardView<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(id, card)| Ok(CardView::new(*id, &card.synopsis)))
    }
}
