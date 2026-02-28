use exocortex_damo::{Card as _, Provider};

pub(crate) fn open_or_prepopulate<A, P: Provider<A>>(args: A) -> Result<P, P::UpdateError> {
    let (mut prov, is_new) = P::open(args)?;
    if is_new {
        let cardid = prov.new_card()?;
        let card = prov.open_card_mut(cardid)?;
        card.set_synopsis("Hello World")?;
    }
    Ok(prov)
}
