use extension_traits::extension;
use redb::{
    ReadTransaction, ReadableTable as _, ReadableTableMetadata as _, TableDefinition,
    WriteTransaction,
};

use crate::{DbError, Id, Result};

#[extension(pub(crate) trait WriteTransactionStore)]
impl WriteTransaction {
    fn store<S: Store>(&self, value: S) -> Result<Id<S::IdType>> {
        value.store_into(self)
    }
}

#[extension(pub(crate) trait ReadTransactionLoad)]
impl ReadTransaction {
    fn load<L: Load>(&self, id: Id<L::IdType>) -> Result<L> {
        L::load(self, id)
    }

    fn scan<L: Load>(&self) -> Result<Vec<(Id<L::IdType>, L)>> {
        L::scan(self)
    }
}

pub(crate) trait Tabular {
    type IdType;
    type RedVal: RedVal;

    fn table_definition() -> TableDefinition<'static, Id<Self::IdType>, Self::RedVal>;
}

pub(crate) trait RedVal: redb::Value + 'static {}

impl<B> RedVal for B where B: redb::Value + 'static {}

pub(crate) trait Store: Tabular + StoreColumnar<RedValStore = Self::RedVal> {
    fn store_into(self, txn: &WriteTransaction) -> Result<Id<Self::IdType>> {
        let redval = self.store_columnar(txn)?;

        let mut tab = txn.open_table(Self::table_definition())?;
        let idnum = tab.len()?;
        let id = Id::new(idnum);
        tab.insert(id, redval)?;

        Ok(id)
    }
}

impl<B> Store for B where B: Tabular + StoreColumnar<RedValStore = Self::RedVal> {}

pub(crate) trait Load: Tabular + LoadColumnar<RedValLoad = Self::RedVal> {
    fn load(txn: &ReadTransaction, id: Id<Self::IdType>) -> Result<Self> {
        let tab = txn.open_table(Self::table_definition())?;
        let optg = tab.get(id)?;
        let guard = optg.ok_or_else(|| DbError::from(id))?;
        let rowval = guard.value();
        let value = Self::load_columnar(txn, rowval)?;
        Ok(value)
    }

    /// # TODO
    ///
    /// Figure out how to make this an iterator by working through lifetime shenanigans
    fn scan(txn: &ReadTransaction) -> Result<Vec<(Id<Self::IdType>, Self)>> {
        let mut items = vec![];
        let tab = txn.open_table(Self::table_definition())?;
        for kvgres in tab.iter()? {
            let (kg, vg) = kvgres?;
            let id = kg.value();
            let rowval = vg.value();
            let value = Self::load_columnar(txn, rowval)?;
            items.push((id, value));
        }
        Ok(items)
    }
}

impl<B> Load for B where B: Tabular + LoadColumnar<RedValLoad = Self::RedVal> {}

pub(crate) trait StoreColumnar: Sized {
    type RedValStore: RedVal;

    fn store_columnar(
        self,
        txn: &WriteTransaction,
    ) -> Result<<Self::RedValStore as redb::Value>::SelfType<'static>>;
}

pub(crate) trait LoadColumnar: Sized {
    type RedValLoad: RedVal;

    fn load_columnar<'a>(
        txn: &ReadTransaction,
        v: <Self::RedValLoad as redb::Value>::SelfType<'a>,
    ) -> Result<Self>;
}
