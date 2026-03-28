use std::collections::BTreeMap;

use exocortex_db::messages::LogScanItems;
use exocortex_db::{CardId, Timestamp};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct CardAgg {
    pub(crate) id: CardId,
    pub(crate) ctime: Timestamp,
    pub(crate) mtime: Timestamp,
    pub(crate) synopsis: String,
}

pub(crate) fn aggregate_card_modifications(
    modifications: &LogScanItems,
) -> impl Iterator<Item = CardAgg> {
    use exocortex_db::messages::CardModifyG::*;

    let mut bt = BTreeMap::default();

    for (_, cardmod) in modifications {
        let mtime = cardmod.time;

        match &cardmod.val {
            Create(id) => {
                let id = *id;
                assert!(
                    bt.insert(
                        id,
                        CardAgg {
                            id,
                            ctime: mtime,
                            mtime,
                            synopsis: "".to_string()
                        }
                    )
                    .is_none()
                );
            }
            SetSynopsis(css) => {
                let agg = bt.get_mut(&css.card).unwrap();
                agg.mtime = mtime;
                agg.synopsis = css.synopsis.clone();
            }
        }
    }

    bt.into_values()
}
