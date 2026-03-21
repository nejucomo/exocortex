use std::any::Any;

use derive_more::From;
use thiserror::Error;

pub type ReqRepRes<T, E> = std::result::Result<T, ReqRepError<E>>;

#[derive(Debug, Error, From)]
pub enum ReqRepError<E> {
    #[error(transparent)]
    Custom(E),
    #[from(Box<dyn Any + Send + 'static>)]
    ChildPanic(Box<dyn Any + Send + 'static>),
}
