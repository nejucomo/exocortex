use exocortex_handler::SyncHandler;
use exocortex_lid::WithId;
use exocortex_memory::modifications::ThopSetSynopsis;
use exocortex_memory::queries::{ThopCount, ThopCounted};
use exocortex_memory::{Provider, Reply, Request, Thop};
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
    // TODO: Describe the shortcut based on the live system.
    "Append your own thop with <cmd>-<enter>. Try it now.",
];

pub(crate) fn prepopulate<P>(db: &mut P) -> Result<(), <P as SyncHandler<Request>>::SyncError>
where
    P: Provider + SyncHandler<Request, Reply = Reply>,
{
    if let ThopCounted(0) = db.handle_subrequest::<_, ThopCounted>(ThopCount)? {
        log::debug!("Prepopulating the database...");
        for thoptxt in CANNED_THOPS.iter() {
            let WithId {
                id: _,
                value: thopmod,
            } = db.handle_subrequest(Thop)?;

            let thop = thopmod.thop;
            let c2 = db.handle_subrequest(ThopSetSynopsis::new(thop, thoptxt.to_string()))?;
            assert_eq!(thop, c2);
        }
    }
    Ok(())
}
