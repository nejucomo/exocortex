//! Utilities for debugging values to [log::debug]
#![deny(unsafe_code, missing_docs)]

use std::fmt::Debug;

/// Debug any [Debug] type into the dbg log
pub trait LogDebug: Debug {
    /// Debug the value with `label` as a prefix; returning the value
    fn dbg(self, label: &'static str) -> Self
    where
        Self: Sized,
    {
        self.dbg_ref(label);
        self
    }

    /// Debug a reference with `label` as a prefix; returning that reference
    fn dbg_ref(&self, label: &'static str) -> &Self {
        log::debug!("{label}: {:?}", self);
        self
    }

    /// Debug a `mut` reference with `label` as a prefix; returning that reference
    fn dbg_mut(&mut self, label: &'static str) -> &mut Self {
        log::debug!("{label}: {:?}", self);
        self
    }
}

impl<T> LogDebug for T where T: std::fmt::Debug {}
