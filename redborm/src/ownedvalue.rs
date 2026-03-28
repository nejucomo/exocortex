use redb::{Key, Value};

/// Any [Value] whose read/write [Value::SelfType] is itself is an [OwnedValue]
///
/// The [Value] trait is finicky due to supporting a variety of uses; this hardcodes an inefficient subcase of owned values to simplify usage as an expedient.
pub trait OwnedValue: for<'a> Value<SelfType<'a> = Self> + Sized + 'static {}

impl<B> OwnedValue for B where B: for<'a> Value<SelfType<'a> = Self> + Sized + 'static {}

/// Any [Key] which is an [OwnedValue] is an [OwnedKey]
pub trait OwnedKey: Key + OwnedValue {}

impl<B> OwnedKey for B where B: Key + OwnedValue {}
