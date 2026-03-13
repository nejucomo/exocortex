use redb::TableDefinition;

use crate::Id;

pub(super) struct TableDefinitions {
    pub(super) synopsis: TableDefinition<'static, Id, &'static str>,
}

impl Default for TableDefinitions {
    fn default() -> Self {
        Self {
            synopsis: TableDefinition::new("synopsis_v0"),
        }
    }
}

impl std::fmt::Debug for TableDefinitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableDefinitions")
            .field("synopsis", &"TableDefinition{ Id -> &'static str }")
            .finish()
    }
}
