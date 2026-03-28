use exocortex_db::entities::CardSetSynopsisV0;
use exocortex_db::messages::{CardCreate, DbIsEmpty};
use exocortex_db::{DatabaseThreadService, DbResult};
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

pub(crate) fn prepopulate(db: &mut DatabaseThreadService) -> DbResult<()> {
    if db.request(DbIsEmpty)? {
        for cardtxt in CANNED_CARDS.iter().rev() {
            let card = db.request(CardCreate)?;
            let c2 = db.request(CardSetSynopsisV0::new(card, cardtxt.to_string()))?;
            assert_eq!(card, c2);
        }
    }
    Ok(())
}
