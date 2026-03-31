use exocortex_db::entities::ThopSetSynopsisV0;
use exocortex_db::messages::{DbIsEmpty, ThopCreate};
use exocortex_db::{DatabaseThreadService, DbResult};
use indoc::indoc;

const CANNED_THOPS: &[&str] = &[
    "# Welcome to exocortex!",
    "The previous welcome line and this line are each separate _thops_: *th*oughts *o*n *p*aper.",
    indoc! { r#"
        Thops can be longer. Try clicking this.

        # Thops are Markdown

        Each thop is a markdown document, and can include markdown like:

        - lists, like this one
        - _italics_, **bold**, `fixed width`
        - [ ] unchecked boxes
        - [x] checked boxes
        - ... and so on.

        However, this CommonMark-style markdown does not support:

        - <u>inline html for underline</u>
    "# },
];

pub(crate) fn prepopulate(db: &mut DatabaseThreadService) -> DbResult<()> {
    if db.request(DbIsEmpty)? {
        log::debug!("Prepopulating the database...");
        for thoptxt in CANNED_THOPS.iter() {
            let thop = db.request(ThopCreate)?;
            let c2 = db.request(ThopSetSynopsisV0::new(thop, thoptxt.to_string()))?;
            assert_eq!(thop, c2);
        }
    }
    Ok(())
}
