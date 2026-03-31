//! Modify request type
use derive_more::TryIntoError;
use exocortex_redborm::enumvalue::EnumVariantCode;
use exocortex_redborm::ext::{ReadTransactionExt as _, WriteTransactionExt as _};
use exocortex_redborm::{Id, Load, OrmError, OrmResult, Store};
use redb::{ReadTransaction, WriteTransaction};

use crate::entities::{ThopModificationV0, ThopSetSynopsisV0};
use crate::messages::{Request, ThopCreate};
use crate::{ThopId, Timestamp, Timestamped};

/// A request to modify a thop
pub type ThopModify = ThopModifyG<ThopCreate, ThopSetSynopsisV0>;

impl From<ThopCreate> for ThopModify {
    fn from(cc: ThopCreate) -> Self {
        ThopModifyG::Create(cc)
    }
}

impl TryFrom<ThopModify> for ThopCreate {
    type Error = TryIntoError<ThopModify>;

    fn try_from(cm: ThopModify) -> Result<Self, Self::Error> {
        if let ThopModifyG::Create(cc) = cm {
            Ok(cc)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

impl From<ThopSetSynopsisV0> for ThopModify {
    fn from(css: ThopSetSynopsisV0) -> Self {
        ThopModifyG::SetSynopsis(css)
    }
}

impl TryFrom<ThopModify> for ThopSetSynopsisV0 {
    type Error = TryIntoError<ThopModify>;

    fn try_from(cm: ThopModify) -> Result<Self, Self::Error> {
        if let ThopModifyG::SetSynopsis(css) = cm {
            Ok(css)
        } else {
            Err(TryIntoError::new(cm, "FIXME", "FIXME"))
        }
    }
}

/// The result of modifying a thop
pub type ThopModified = ThopModifyG<ThopId, ThopSetSynopsisV0>;

/*
impl ThopModified {
    /// Get the [ThopId] of the modified thop
    pub fn thop_id(&self) -> ThopId {
        use ThopModifyG::*;

        match self {
            Create(thop) => *thop,
            SetSynopsis(css) => css.thop,
        }
    }
}
*/

impl Request for ThopModify {
    type Reply = ThopId;
}

/// A thop modification request or db [Store]/[Load]
#[derive(Debug)]
pub enum ThopModifyG<Create, SetSynopsis> {
    #[allow(missing_docs)]
    Create(Create),
    #[allow(missing_docs)]
    SetSynopsis(SetSynopsis),
}

impl Store for ThopModify {
    type KOV = ThopId;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        use ThopModifyG::*;

        let time = Timestamp::now();

        let cmv0 = match self {
            Create(ThopCreate) => {
                let thop = ThopCreate.store_into(txn)?;
                let variant: EnumVariantCode = 0;
                ThopModificationV0 {
                    thop,
                    time,
                    enumcol: (variant, thop.transmute()).into(),
                }
            }
            SetSynopsis(css) => {
                let thop = css.thop;
                let variant: EnumVariantCode = 1;
                let cssid = css.store_into(txn)?;
                ThopModificationV0 {
                    thop,
                    time,
                    enumcol: (variant, cssid.transmute()).into(),
                }
            }
        };

        let id = txn.store(cmv0)?;
        log::trace!("Recording: {id:?} <- {cmv0:#?}");
        Ok(cmv0.thop)
    }
}

impl Load for Timestamped<ThopModified> {
    type KOV = Id<ThopModificationV0>;

    fn load_from(txn: &ReadTransaction, kov: Id<ThopModificationV0>) -> OrmResult<Self> {
        let cmv0 = txn.load(kov)?;
        Self::load_via(txn, cmv0)
    }

    fn scan_from<F>(txn: &ReadTransaction, mut take_item: F) -> OrmResult<()>
    where
        F: FnMut(Id<ThopModificationV0>, Self) -> OrmResult<()>,
    {
        txn.scan(|id: Id<ThopModificationV0>, cmv0: ThopModificationV0| {
            let tscm = Self::load_via(txn, cmv0)?;
            take_item(id, tscm)?;
            Ok(())
        })
    }
}

impl Timestamped<ThopModified> {
    fn load_via(txn: &ReadTransaction, cmv0: ThopModificationV0) -> OrmResult<Self> {
        use ThopModifyG::*;

        let ThopModificationV0 {
            thop,
            time,
            enumcol,
        } = cmv0;

        match enumcol.variant {
            0 => Ok(time.stamp(Create(thop))),
            1 => txn
                .load(enumcol.referent.transmute())
                .map(SetSynopsis)
                .map(|v| time.stamp(v)),
            other => Err(OrmError::load_invalid_enum_variant::<Self>(other)),
        }
    }
}
