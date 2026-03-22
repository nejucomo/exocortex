use std::borrow::Borrow;

use extension_traits::extension;
use redb::{ReadableTableMetadata as _, TableDefinition, WriteTransaction};

use crate::entities::Card;
use crate::messages::{CardCreate, CardModification, CardModify, CardSetSynopsis, Modify};
use crate::tables::{EnumValue, Variant};
use crate::{Id, Result, tables};

#[extension(pub(crate) trait WriteTransactionStore)]
impl WriteTransaction {
    fn store<S: Store>(&self, value: &S) -> Result<(Id<S>, S::Aux)> {
        value.store_into(self)
    }
}

pub(crate) trait Store: PreStore {
    fn store_into(&self, txn: &WriteTransaction) -> Result<(Id<Self>, Self::Aux)> {
        let (value, aux) = self.prestore_into(txn)?;

        let mut cctab = txn.open_table(Self::table_definition())?;
        let idnum = cctab.len()?;
        let id = Id::new(idnum);
        cctab.insert(id, value)?;

        Ok((id, aux))
    }

    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value>;
}

pub(crate) trait PreStore {
    type Value: redb::Value + for<'a> Borrow<<Self::Value as redb::Value>::SelfType<'a>>;
    type Aux;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(Self::Value, Self::Aux)>;
}

/// Prevent going cross-eyed with parenthesesitis
type NoAux = ();
const NO_AUX: NoAux = ();

impl Store for Modify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::MODIFY_V0
    }
}

impl PreStore for Modify {
    type Value = EnumValue;
    type Aux = Id<Card>;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(EnumValue, Id<Card>)> {
        use Modify::*;

        match self {
            CardCreate(sub) => {
                let v: Variant = 0;
                let (id, NO_AUX) = txn.store(sub)?;
                Ok(((v, id.transmute()), id.transmute()))
            }

            CardModify(sub) => {
                let v: Variant = 1;
                let (id, card) = txn.store(sub)?;
                Ok(((v, id.transmute()), card))
            }
        }
    }
}

impl Store for CardCreate {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_CREATE_V0
    }
}

impl PreStore for CardCreate {
    type Value = ();
    type Aux = NoAux;

    fn prestore_into(&self, _: &WriteTransaction) -> Result<((), NoAux)> {
        Ok(((), NO_AUX))
    }
}

impl Store for CardModify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_MODIFY_V0
    }
}

impl PreStore for CardModify {
    type Value = (Id<Card>, EnumValue);
    type Aux = Id<Card>;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(Self::Value, Id<Card>)> {
        let (sub, NO_AUX) = self.modif.prestore_into(txn)?;
        Ok(((self.card, sub), self.card))
    }
}

impl PreStore for CardModification {
    type Value = EnumValue;
    type Aux = NoAux;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(Self::Value, NoAux)> {
        use CardModification::*;

        match self {
            SetSynopsis(sub) => {
                let v: Variant = 0;
                let (id, NO_AUX) = txn.store(sub)?;
                Ok(((v, id.transmute()), NO_AUX))
            }
        }
    }
}

impl Store for CardSetSynopsis {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_SET_SYNOPSIS_V0
    }
}

impl PreStore for CardSetSynopsis {
    type Value = String; // TODO: Switch to `&str`
    type Aux = NoAux;

    fn prestore_into(&self, _: &WriteTransaction) -> Result<(Self::Value, NoAux)> {
        Ok((self.0.clone(), NO_AUX))
    }
}
