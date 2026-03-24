use std::borrow::Borrow;

use extension_traits::extension;
use redb::{
    ReadTransaction, ReadableTable as _, ReadableTableMetadata as _, TableDefinition,
    WriteTransaction,
};

use crate::Error::{self, LoadInvalidEnumVariant};
use crate::entities::Card;
use crate::messages::{CardCreate, CardModification, CardModify, CardSetSynopsis, Modify};
use crate::tables::{EnumValue, Variant};
use crate::{Id, Result, tables};

#[extension(pub(crate) trait WriteTransactionStore)]
impl WriteTransaction {
    fn store<S: StoreLoad>(&self, value: &S) -> Result<(Id<S>, S::StoreAux)> {
        value.store_into(self)
    }
}

#[extension(pub(crate) trait ReadTransactionLoad)]
impl ReadTransaction {
    fn load<L>(&self, id: Id<L>) -> Result<L>
    where
        L: StoreLoad,
    {
        L::load_from(self, id)
    }

    fn scan<L>(&self) -> Result<Vec<(Id<L>, L)>>
    where
        L: StoreLoad,
    {
        L::scan(self)
    }
}

pub(crate) trait StoreLoad: SLValue + 'static {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value>;

    fn store_into(&self, txn: &WriteTransaction) -> Result<(Id<Self>, Self::StoreAux)> {
        let (value, aux) = self.prestore_into(txn)?;

        let mut tab = txn.open_table(Self::table_definition())?;
        let idnum = tab.len()?;
        let id = Id::new(idnum);
        tab.insert(id, value)?;

        Ok((id, aux))
    }

    fn load_from(txn: &ReadTransaction, id: Id<Self>) -> Result<Self> {
        let tab = txn.open_table(Self::table_definition())?;
        let optg = tab.get(id)?;
        let guard = optg.ok_or_else(|| Error::from(id))?;
        let rowval = guard.value();
        let value = Self::load_from_value(txn, rowval)?;
        Ok(value)
    }

    /// # TODO
    ///
    /// Figure out how to make this an iterator by working through lifetime shenanigans
    fn scan(txn: &ReadTransaction) -> Result<Vec<(Id<Self>, Self)>> {
        let mut items = vec![];
        let tab = txn.open_table(Self::table_definition())?;
        for kvgres in tab.iter()? {
            let (kg, vg) = kvgres?;
            let id = kg.value();
            let rowval = vg.value();
            let value = Self::load_from_value(txn, rowval)?;
            items.push((id, value));
        }
        Ok(items)
    }
}

pub(crate) trait SLValue: Sized {
    type Value: redb::Value + for<'a> Borrow<<Self::Value as redb::Value>::SelfType<'a>>;
    type StoreAux;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(Self::Value, Self::StoreAux)>;

    fn load_from_value<'a>(
        txn: &ReadTransaction,
        v: <Self::Value as redb::Value>::SelfType<'a>,
    ) -> Result<Self>;
}

/// Prevent going cross-eyed with parenthesesitis
type NoAux = ();
const NO_AUX: NoAux = ();

impl StoreLoad for Modify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::MODIFY_V0
    }
}

impl SLValue for Modify {
    type Value = EnumValue;
    type StoreAux = Id<Card>;

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

    fn load_from_value<'a>(
        txn: &ReadTransaction,
        v: <Self::Value as redb::Value>::SelfType<'a>,
    ) -> Result<Self> {
        use Modify::*;

        let (variant, eid) = v;
        match variant {
            0 => txn.load(eid.transmute()).map(CardCreate),
            1 => txn.load(eid.transmute()).map(CardModify),
            other => Err(LoadInvalidEnumVariant {
                type_name: std::any::type_name::<Self>(),
                variant_code: other,
            }),
        }
    }
}

impl StoreLoad for CardCreate {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_CREATE_V0
    }
}

impl SLValue for CardCreate {
    type Value = ();
    type StoreAux = NoAux;

    fn prestore_into(&self, _: &WriteTransaction) -> Result<((), NoAux)> {
        Ok(((), NO_AUX))
    }

    fn load_from_value<'a>(
        _: &ReadTransaction,
        (): <Self::Value as redb::Value>::SelfType<'a>,
    ) -> Result<Self> {
        Ok(CardCreate)
    }
}

impl StoreLoad for CardModify {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_MODIFY_V0
    }
}

impl SLValue for CardModify {
    type Value = (Id<Card>, EnumValue);
    type StoreAux = Id<Card>;

    fn prestore_into(&self, txn: &WriteTransaction) -> Result<(Self::Value, Id<Card>)> {
        let (sub, NO_AUX) = self.modif.prestore_into(txn)?;
        Ok(((self.card, sub), self.card))
    }

    fn load_from_value<'a>(
        txn: &ReadTransaction,
        (card, enval): <Self::Value as redb::Value>::SelfType<'a>,
    ) -> Result<Self> {
        let modif = CardModification::load_from_value(txn, enval)?;
        Ok(CardModify { card, modif })
    }
}

impl SLValue for CardModification {
    type Value = EnumValue;
    type StoreAux = NoAux;

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

    fn load_from_value<'a>(
        txn: &ReadTransaction,
        v: <Self::Value as redb::Value>::SelfType<'a>,
    ) -> Result<Self> {
        use CardModification::*;

        let (variant, eid) = v;
        match variant {
            0 => txn.load(eid.transmute()).map(SetSynopsis),
            other => Err(LoadInvalidEnumVariant {
                type_name: std::any::type_name::<Self>(),
                variant_code: other,
            }),
        }
    }
}

impl StoreLoad for CardSetSynopsis {
    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        tables::CARD_SET_SYNOPSIS_V0
    }
}

impl SLValue for CardSetSynopsis {
    type Value = String; // TODO: Switch to `&str`
    type StoreAux = NoAux;

    fn prestore_into(&self, _: &WriteTransaction) -> Result<(Self::Value, NoAux)> {
        Ok((self.0.clone(), NO_AUX))
    }

    fn load_from_value(_: &ReadTransaction, synstr: String) -> Result<Self> {
        Ok(CardSetSynopsis(synstr))
    }
}
