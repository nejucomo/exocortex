use exocortex_db::entities::ThopSetSynopsisV0;
use exocortex_db::messages::{ThopCreate, DbIsEmpty};
use exocortex_db::{DatabaseThreadService, DbResult};
use indoc::indoc;

const CANNED_THOPS: &[&str] = &[
    "Welcome to exocortex!",
    "The previous welcome line and this line are each separate _thops_.",
    indoc! { r#"
        Thops can be longer. [TODO: implement: "Click this."]

        Each thop is a markdown document, and can include markdown like:

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
        for thoptxt in CANNED_THOPS.iter().rev() {
            let thop = db.request(ThopCreate)?;
            let c2 = db.request(ThopSetSynopsisV0::new(thop, thoptxt.to_string()))?;
            assert_eq!(thop, c2);
        }
    }
    Ok(())
}
