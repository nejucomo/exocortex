#![deny(unsafe_code)]

mod page;
mod path;

pub mod error;
pub use self::page::Page;
pub use self::path::{PagePath, PagePathRef};
