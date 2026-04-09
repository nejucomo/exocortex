use std::cmp::Ordering;

use redb::{Key, TypeName, Value};

use crate::{Id, ValueWithId as _, WithId};

impl<T: ?Sized + 'static> Value for Id<T> {
    type SelfType<'a>
        = Id<T>
    where
        Self: 'a;
    type AsBytes<'a>
        = [u8; 8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(8)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        Id::from(u64_from_be_slice(data))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.n.to_be_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new(std::any::type_name::<Self>())
    }
}

impl<T: ?Sized + 'static> Key for Id<T> {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        let a = u64_from_be_slice(data1);
        let b = u64_from_be_slice(data2);
        a.cmp(&b)
    }
}

impl<T: Value + 'static> Value for WithId<T> {
    type SelfType<'a>
        = WithId<T::SelfType<'a>>
    where
        Self: 'a;

    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        let id_size = Id::<T>::fixed_width()?;
        let t_size = T::fixed_width()?;
        Some(id_size + t_size)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let (id, value) = <(Id<T>, T)>::from_bytes(data);
        value.with_id(id.transmute())
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        let id_bytes = Id::<T>::as_bytes(&value.id.transmute());
        let value_bytes = T::as_bytes(&value.value);
        let mut bytes = Vec::with_capacity(id_bytes.len() + value_bytes.as_ref().len());
        bytes.extend_from_slice(&id_bytes);
        bytes.extend_from_slice(value_bytes.as_ref());
        bytes
    }

    fn type_name() -> TypeName {
        TypeName::new(std::any::type_name::<Self>())
    }
}

fn u64_from_be_slice(data: &[u8]) -> u64 {
    u64::from_be_bytes(data.try_into().expect("Id<T> key must be 8 bytes"))
}
