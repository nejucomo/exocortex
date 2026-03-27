use derive_more::From;
use thiserror::Error;

/// The database error type [Result](std::result::Result) shorthand
pub type OrmResult<T> = std::result::Result<T, OrmError>;

/// The database error type
#[derive(Debug, Error, From)]
pub enum OrmError {
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

    /// An attempt to load a row with an unknown key
    ///
    /// # Note
    ///
    /// The [String] must describe a key
    #[error("failed to load unkown key: {0}")]
    UnknownKey(String),

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
