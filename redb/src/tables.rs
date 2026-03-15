use redb::TableDefinition;

use crate::Id;
use crate::messages::Card;

pub(crate) struct Tables {
    pub(crate) card_synopsis: TableDefinition<'static, Id<Card>, &'static str>,
}

pub(crate) const TABLES: Tables = Tables {
    card_synopsis: TableDefinition::new("card_synopsis_v0"),
};
