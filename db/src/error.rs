use derive_more::From;
use thiserror::Error;

use crate::Id;

/// The database error type [Result](std::result::Result) shorthand
pub type Result<T> = std::result::Result<T, Error>;

/// The database error type
#[derive(Debug, Error, From)]
pub enum Error {
    /// An internal [redb::Error]
    #[from(
        redb::CommitError,
        redb::DatabaseError,
        redb::Error,
        redb::StorageError,
        redb::TableError,
        redb::TransactionError,
        std::io::Error
    )]
    #[error(transparent)]
    Redb(redb::Error),

    /// An attempt to load an entity with an unknown [Id]
    ///
    /// # Note
    ///
    /// The [String] must describe an [Id]`<T>`
    #[error("failed to load {0}: unknown")]
    MissingEntity(String),

    /// An attempt to load an enum entity with an unknown variant
    ///
    /// This error would indicate a bug in DB serialization code, perhaps due to unaccounted for schema skew.
    #[error("invalid enum variant {variant_code:?} while loading {type_name} from db")]
    LoadInvalidEnumVariant {
        /// The type name of the entity
        type_name: &'static str,
        /// The unrecognized variant
        variant_code: u32,
    },
}

impl<T> From<Id<T>> for Error {
    fn from(id: Id<T>) -> Self {
        Error::MissingEntity(format!("{id:?}"))
    }
}
