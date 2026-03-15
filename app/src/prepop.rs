use exocortex_redb::messages::RepSpec::{Modified, Queried};
use exocortex_redb::messages::{
    CardCreate, CardModify, CardSetSynopsis, CardUpdated::Created, DbIsEmpty, Queried::DbWasEmpty,
};
use exocortex_redb::{DbResult, ExoDb};
use indoc::indoc;

const CANNED_CARDS: &[&str] = &[
    "# Welcome to exocortex!",
    "# The previous welcome line and this line are each separate `cards`.",
    indoc! { r#"
        # Cards can be longer:

        Each card is a markdown document, and can include markdown like:

        - lists, like this one
        - _italics_, **bold**, `fixed width`
        - [ ] unchecked boxes
        - [x] checked boxes
        - ... and so on.

        However, this markdown does not support:

        - <u>inline html for underline</u>
    "# },
    indoc! { r#"
        # This is the `log` `view`

        Each update to a card is displayed in reverse chronological order.
    "# },
];

pub(crate) fn prepopulate(db: &mut ExoDb) -> DbResult<()> {
    if matches!(db.request(DbIsEmpty)?, Queried(DbWasEmpty(true))) {
        for cardtxt in CANNED_CARDS.iter().rev() {
            let card = match db.request(CardCreate)? {
                Modified(Created(c)) => c,
                other => panic!("incoherent db response: {other:?}"),
            };

            db.request(CardModify::new(card, CardSetSynopsis(cardtxt.to_string())))?;
        }
    }
    Ok(())
}
