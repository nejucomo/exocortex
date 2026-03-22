use std::borrow::Borrow;

use redb::{ReadableTableMetadata as _, TableDefinition, WriteTransaction};

use crate::entities::Card;
use crate::messages::{CardCreate, CardModification, CardModify, CardSetSynopsis, Modify};
use crate::tables::{EnumValue, Variant};
use crate::{Id, Result, tables};

pub(crate) trait Save: SaveValue {
    fn save_into(&self, txn: &WriteTransaction) -> Result<(Id<Self>, Self::Aux)> {
        let (value, aux) = self.prepare_value(txn)?;

        let mut cctab = txn.open_table(Self::table_definition())?;
        let idnum = cctab.len()?;
        let id = Id::new(idnum);
        cctab.insert(id, value)?;

        Ok((id, aux))
    }

    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value>;
}

pub(crate) trait SaveValue {
    type Value: redb::Value + for<'a> Borrow<<Self::Value as redb::Value>::SelfType<'a>>;
    type Aux;

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<(Self::Value, Self::Aux)>;
}

/// Prevent going cross-eyed with parenthesesitis
type NoAux = ();
const NO_AUX: NoAux = ();

impl Save for Modify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::MODIFY_V0
    }
}

impl SaveValue for Modify {
    type Value = EnumValue;
    type Aux = Id<Card>;

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<(EnumValue, Id<Card>)> {
        use Modify::*;

        match self {
            CardCreate(sub) => {
                let v: Variant = 0;
                let (id, NO_AUX) = sub.save_into(txn)?;
                Ok(((v, id.transmute()), id.transmute()))
            }

            CardModify(sub) => {
                let v: Variant = 1;
                let (id, card) = sub.save_into(txn)?;
                Ok(((v, id.transmute()), card))
            }
        }
    }
}

impl Save for CardCreate {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_CREATE_V0
    }
}

impl SaveValue for CardCreate {
    type Value = ();
    type Aux = NoAux;

    fn prepare_value(&self, _: &WriteTransaction) -> Result<((), NoAux)> {
        Ok(((), NO_AUX))
    }
}

impl Save for CardModify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_MODIFY_V0
    }
}

impl SaveValue for CardModify {
    type Value = (Id<Card>, EnumValue);
    type Aux = Id<Card>;

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<(Self::Value, Id<Card>)> {
        let (sub, NO_AUX) = self.modif.prepare_value(txn)?;
        Ok(((self.card, sub), self.card))
    }
}

impl SaveValue for CardModification {
    type Value = EnumValue;
    type Aux = NoAux;

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<(Self::Value, NoAux)> {
        use CardModification::*;

        match self {
            SetSynopsis(sub) => {
                let v: Variant = 0;
                let (id, NO_AUX) = sub.save_into(txn)?;
                Ok(((v, id.transmute()), NO_AUX))
            }
        }
    }
}

impl Save for CardSetSynopsis {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_SET_SYNOPSIS_V0
    }
}

impl SaveValue for CardSetSynopsis {
    type Value = String; // TODO: Switch to `&str`
    type Aux = NoAux;

    fn prepare_value(&self, _: &WriteTransaction) -> Result<(Self::Value, NoAux)> {
        Ok((self.0.clone(), NO_AUX))
    }
}
