use exocortex_damo::{Card as _, Provider};

pub(crate) fn prepopulated<P: Provider>(mut prov: P) -> Result<P, P::UpdateError> {
    if prov.is_empty() {
        let cardid = prov.new_card()?;
        let card = prov.open_card_mut(cardid)?;
        card.set_synopsis("Hello World")?;
    }
    Ok(prov)
}
