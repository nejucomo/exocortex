//! Modify request type
use exocortex_redborm::enumvalue::EnumVariantCode;
use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_redborm::{Id, Load, OrmError, OrmResult, Store};
use redb::{ReadTransaction, WriteTransaction};

use crate::entities::{CardModificationV0, CardSetSynopsisV0, CardV0};
use crate::messages::{CardCreate, Request};
use crate::{Timestamp, Timestamped};

/// A request to modify a card
pub type CardModify = CardModifyG<CardCreate, CardSetSynopsisV0>;

/// The result of modifying a card
pub type CardModified = CardModifyG<Id<CardV0>, CardSetSynopsisV0>;

impl Request for CardModify {
    type Reply = Id<CardV0>;
}

/// A card modification request or db [Store]/[Load]
#[derive(Debug)]
pub enum CardModifyG<Create, SetSynopsis> {
    #[allow(missing_docs)]
    Create(Create),
    #[allow(missing_docs)]
    SetSynopsis(SetSynopsis),
}

impl Store for CardModify {
    type KOV = Id<CardV0>;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        use CardModifyG::*;

        let time = Timestamp::now();

        let cmv0 = match self {
            Create(CardCreate) => {
                let card = CardCreate.store_into(txn)?;
                let variant: EnumVariantCode = 0;
                CardModificationV0 {
                    card,
                    time,
                    enumcol: (variant, card.transmute()).into(),
                }
            }
            SetSynopsis(css) => {
                let card = css.card;
                let variant: EnumVariantCode = 1;
                let cssid = css.store_into(txn)?;
                CardModificationV0 {
                    card,
                    time,
                    enumcol: (variant, cssid.transmute()).into(),
                }
            }
        };

        let id = txn.store(cmv0)?;
        log::trace!("Recording: {id:?} <- {cmv0:#?}");
        Ok(cmv0.card)
    }
}

impl Load for Timestamped<CardModified> {
    type KOV = Id<CardModificationV0>;

    fn load_from(txn: &ReadTransaction, kov: Id<CardModificationV0>) -> OrmResult<Self> {
        let cmv0 = txn.load(kov)?;
        Self::load_via(txn, cmv0)
    }

    fn scan_from<F>(txn: &ReadTransaction, mut take_item: F) -> OrmResult<()>
    where
        F: FnMut(Id<CardModificationV0>, Self) -> OrmResult<()>,
    {
        txn.scan(|id: Id<CardModificationV0>, cmv0: CardModificationV0| {
            let tscm = Self::load_via(txn, cmv0)?;
            take_item(id, tscm)?;
            Ok(())
        })
    }
}

impl Timestamped<CardModified> {
    fn load_via(txn: &ReadTransaction, cmv0: CardModificationV0) -> OrmResult<Self> {
        use CardModifyG::*;

        let CardModificationV0 {
            card,
            time,
            enumcol,
        } = cmv0;

        match enumcol.variant {
            0 => Ok(time.stamp(Create(card))),
            1 => txn
                .load(enumcol.referent.transmute())
                .map(SetSynopsis)
                .map(|v| time.stamp(v)),
            other => Err(OrmError::load_invalid_enum_variant::<Self>(other)),
        }
    }
}
