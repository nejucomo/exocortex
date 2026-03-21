//! Marker types for database entities; referenced via `Id<EntityType>`

/// A type-disambiguation placeholder for `Id<Card>`
#[derive(Copy, Clone, Debug)]
pub enum Card {}
