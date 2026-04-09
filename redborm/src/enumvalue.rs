//! Support for normalized enum db storage
use derive_more::{From, Into};
use derive_new::new;
use exocortex_lid::Id;
use redb_derive::Value;

/// A "columnar" [Value](redb::Value) representing an normalized enum
#[derive(Copy, Clone, Debug, From, Into, new, Value)]
pub struct EnumColumnar {
    /// The enum variant
    pub variant: EnumVariantCode,
    /// The enum referent [Id]
    pub referent: Id<EnumReferent>,
}

/// The representation of an enum variant
pub type EnumVariantCode = u32;

/// An uninhabitable type for marking the [Id] of an enum referent
pub enum EnumReferent {}
