//! Modify request type
use derive_more::TryIntoError;
use exocortex_redborm::enumvalue::EnumVariantCode;
use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_redborm::{Id, Load, OrmError, OrmResult, Store};
use redb::{ReadTransaction, WriteTransaction};

use crate::entities::{BlurbModificationV0, BlurbSetSynopsisV0};
use crate::messages::{BlurbCreate, Request};
use crate::{BlurbId, Timestamp, Timestamped};

/// A request to modify a blurb
pub type BlurbModify = BlurbModifyG<BlurbCreate, BlurbSetSynopsisV0>;

impl From<BlurbCreate> for BlurbModify {
    fn from(cc: BlurbCreate) -> Self {
        BlurbModifyG::Create(cc)
    }
}

impl TryFrom<BlurbModify> for BlurbCreate {
    type Error = TryIntoError<BlurbModify>;

    fn try_from(cm: BlurbModify) -> Result<Self, Self::Error> {
        if let BlurbModifyG::Create(cc) = cm {
            Ok(cc)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

impl From<BlurbSetSynopsisV0> for BlurbModify {
    fn from(css: BlurbSetSynopsisV0) -> Self {
        BlurbModifyG::SetSynopsis(css)
    }
}

impl TryFrom<BlurbModify> for BlurbSetSynopsisV0 {
    type Error = TryIntoError<BlurbModify>;

    fn try_from(cm: BlurbModify) -> Result<Self, Self::Error> {
        if let BlurbModifyG::SetSynopsis(css) = cm {
            Ok(css)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

/// The result of modifying a blurb
pub type BlurbModified = BlurbModifyG<BlurbId, BlurbSetSynopsisV0>;

/*
impl BlurbModified {
    /// Get the [BlurbId] of the modified blurb
    pub fn blurb_id(&self) -> BlurbId {
        use BlurbModifyG::*;

        match self {
            Create(blurb) => *blurb,
            SetSynopsis(css) => css.blurb,
        }
    }
}
*/

impl Request for BlurbModify {
    type Reply = BlurbId;
}

/// A blurb modification request or db [Store]/[Load]
#[derive(Debug)]
pub enum BlurbModifyG<Create, SetSynopsis> {
    #[allow(missing_docs)]
    Create(Create),
    #[allow(missing_docs)]
    SetSynopsis(SetSynopsis),
}

impl Store for BlurbModify {
    type KOV = BlurbId;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        use BlurbModifyG::*;

        let time = Timestamp::now();

        let cmv0 = match self {
            Create(BlurbCreate) => {
                let blurb = BlurbCreate.store_into(txn)?;
                let variant: EnumVariantCode = 0;
                BlurbModificationV0 {
                    blurb,
                    time,
                    enumcol: (variant, blurb.transmute()).into(),
                }
            }
            SetSynopsis(css) => {
                let blurb = css.blurb;
                let variant: EnumVariantCode = 1;
                let cssid = css.store_into(txn)?;
                BlurbModificationV0 {
                    blurb,
                    time,
                    enumcol: (variant, cssid.transmute()).into(),
                }
            }
        };

        let id = txn.store(cmv0)?;
        log::trace!("Recording: {id:?} <- {cmv0:#?}");
        Ok(cmv0.blurb)
    }
}

impl Load for Timestamped<BlurbModified> {
    type KOV = Id<BlurbModificationV0>;

    fn load_from(txn: &ReadTransaction, kov: Id<BlurbModificationV0>) -> OrmResult<Self> {
        let cmv0 = txn.load(kov)?;
        Self::load_via(txn, cmv0)
    }

    fn scan_from<F>(txn: &ReadTransaction, mut take_item: F) -> OrmResult<()>
    where
        F: FnMut(Id<BlurbModificationV0>, Self) -> OrmResult<()>,
    {
        txn.scan(|id: Id<BlurbModificationV0>, cmv0: BlurbModificationV0| {
            let tscm = Self::load_via(txn, cmv0)?;
            take_item(id, tscm)?;
            Ok(())
        })
    }
}

impl Timestamped<BlurbModified> {
    fn load_via(txn: &ReadTransaction, cmv0: BlurbModificationV0) -> OrmResult<Self> {
        use BlurbModifyG::*;

        let BlurbModificationV0 {
            blurb,
            time,
            enumcol,
        } = cmv0;

        match enumcol.variant {
            0 => Ok(time.stamp(Create(blurb))),
            1 => txn
                .load(enumcol.referent.transmute())
                .map(SetSynopsis)
                .map(|v| time.stamp(v)),
            other => Err(OrmError::load_invalid_enum_variant::<Self>(other)),
        }
    }
}
