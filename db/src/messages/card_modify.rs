//! Modify request type
use derive_more::TryIntoError;
use exocortex_redborm::enumvalue::EnumVariantCode;
use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_redborm::{Id, Load, OrmError, OrmResult, Store};
use redb::{ReadTransaction, WriteTransaction};

use crate::entities::{CardModificationV0, CardSetSynopsisV0};
use crate::messages::{CardCreate, Request};
use crate::{CardId, Timestamp, Timestamped};

/// A request to modify a card
pub type CardModify = CardModifyG<CardCreate, CardSetSynopsisV0>;

impl From<CardCreate> for CardModify {
    fn from(cc: CardCreate) -> Self {
        CardModifyG::Create(cc)
    }
}

impl TryFrom<CardModify> for CardCreate {
    type Error = TryIntoError<CardModify>;

    fn try_from(cm: CardModify) -> Result<Self, Self::Error> {
        if let CardModifyG::Create(cc) = cm {
            Ok(cc)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

impl From<CardSetSynopsisV0> for CardModify {
    fn from(css: CardSetSynopsisV0) -> Self {
        CardModifyG::SetSynopsis(css)
    }
}

impl TryFrom<CardModify> for CardSetSynopsisV0 {
    type Error = TryIntoError<CardModify>;

    fn try_from(cm: CardModify) -> Result<Self, Self::Error> {
        if let CardModifyG::SetSynopsis(css) = cm {
            Ok(css)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

/// The result of modifying a card
pub type CardModified = CardModifyG<CardId, CardSetSynopsisV0>;

/*
impl CardModified {
    /// Get the [CardId] of the modified card
    pub fn card_id(&self) -> CardId {
        use CardModifyG::*;

        match self {
            Create(card) => *card,
            SetSynopsis(css) => css.card,
        }
    }
}
*/

impl Request for CardModify {
    type Reply = CardId;
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
    type KOV = CardId;

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
