use exocortex_db::entities::BlurbSetSynopsisV0;
use exocortex_db::messages::{BlurbCreate, DbIsEmpty};
use exocortex_db::{DatabaseThreadService, DbResult};
use indoc::indoc;

const CANNED_BLURBS: &[&str] = &[
    "Welcome to exocortex!",
    "The previous welcome line and this line are each separate _blurbs_.",
    indoc! { r#"
        Blurbs can be longer. [TODO: implement: "Click this."]

        Each blurb is a markdown document, and can include markdown like:

        - lists, like this one
        - _italics_, **bold**, `fixed width`
        - [ ] unchecked boxes
        - [x] checked boxes
        - ... and so on.

        However, this markdown does not support:

        - <u>inline html for underline</u>
    "# },
];

pub(crate) fn prepopulate(db: &mut DatabaseThreadService) -> DbResult<()> {
    if db.request(DbIsEmpty)? {
        log::debug!("Prepopulating the database...");
        for blurbtxt in CANNED_BLURBS.iter().rev() {
            let blurb = db.request(BlurbCreate)?;
            let c2 = db.request(BlurbSetSynopsisV0::new(blurb, blurbtxt.to_string()))?;
            assert_eq!(blurb, c2);
        }
    }
    Ok(())
}
