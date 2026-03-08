use exocortex_damo::{Card as _, Provider};
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

pub(crate) fn prepopulated<P: Provider>(mut prov: P) -> Result<P, P::UpdateError> {
    if prov.is_empty() {
        for cardtxt in CANNED_CARDS.iter().rev() {
            let cardid = prov.card_new()?;
            let card = prov.open_card_mut(cardid)?;
            card.set_synopsis(cardtxt)?;
        }
    }
    Ok(prov)
}
