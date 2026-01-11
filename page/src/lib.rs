#![deny(unsafe_code)]

mod db;
mod page;
mod path;

pub mod error;
pub use self::db::PageDb;
pub use self::page::Page;
pub use self::path::{PagePath, PagePathRef};
