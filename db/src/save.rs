use std::borrow::Borrow;

use redb::{ReadableTableMetadata as _, TableDefinition, WriteTransaction};

use crate::messages::{CardCreate, Modify};
use crate::{Id, Result};

pub(crate) trait Save {
    fn save_into(&self, txn: &WriteTransaction) -> Result<Id<Self>> {
        let value = self.prepare_value(txn)?;

        let mut cctab = txn.open_table(Self::table_definition())?;
        let idnum = cctab.len()?;
        let id = Id::new(idnum);
        cctab.insert(id, value)?;

        Ok(id)
    }

    fn table_name() -> &'static str;

    fn table_definition() -> TableDefinition<'static, Id<Self>, Self::Value> {
        TableDefinition::new(Self::table_name())
    }

    type Value: redb::Value + for<'a> Borrow<<Self::Value as redb::Value>::SelfType<'a>>;

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<Self::Value>;
}

impl Save for Modify {
    fn table_name() -> &'static str {
        "MODIFY_V0"
    }

    type Value = (u32, u64);

    fn prepare_value(&self, txn: &WriteTransaction) -> Result<Self::Value> {
        use Modify::*;

        match self {
            CardCreate(sub) => {
                let variant = 0;
                let id = sub.save_into(txn)?;
                Ok((variant, id.unwrap()))
            }

            CardModify(sub) => {
                let variant = 0;
                let id = sub.save_into(txn)?;
                Ok((variant, id.unwrap()))
            }
        }
    }
}

impl Save for CardCreate {
    fn table_name() -> &'static str {
        "CARD_CREATE_V0"
    }

    type Value = ();

    fn prepare_value(&self, _: &WriteTransaction) -> Result<Self::Value> {
        Ok(())
    }
}
