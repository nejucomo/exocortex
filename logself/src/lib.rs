//! Utilities for debugging values to [log::debug]
#![deny(unsafe_code, missing_docs)]

use std::fmt::Debug;

use log::Level;

/// Debug any [Debug] type into the dbg log
pub trait LogSelf: Debug {
    /// log `self` at `level` with `label` as a prefix, then return self
    fn log_debug(self, label: &'static str) -> Self
    where
        Self: Sized,
    {
        self.log_debug_ref(label);
        self
    }

    /// log a `self` reference with `label` as a prefix, then return the reference
    fn log_debug_ref(&self, label: &'static str) -> &Self {
        self.log_level(Level::Debug, label);
        self
    }

    /// log a `self` `mut` reference with `label` as a prefix, then return the reference
    fn log_debug_mut(&mut self, label: &'static str) -> &mut Self {
        self.log_debug_ref(label);
        self
    }

    /// log `self` at `level` with `label` as a prefix, then return self
    fn log_warn(self, label: &'static str) -> Self
    where
        Self: Sized,
    {
        self.log_warn_ref(label);
        self
    }

    /// log a `self` reference with `label` as a prefix, then return the reference
    fn log_warn_ref(&self, label: &'static str) -> &Self {
        self.log_level(Level::Warn, label);
        self
    }

    /// log a `self` `mut` reference with `label` as a prefix, then return the reference
    fn log_warn_mut(&mut self, label: &'static str) -> &mut Self {
        self.log_warn_ref(label);
        self
    }

    /// log `self` at `level` with `label` as a prefix, then return self
    fn log_level(self, level: Level, label: &'static str) -> Self
    where
        Self: Sized,
    {
        self.log_level_ref(level, label);
        self
    }

    /// log a `self` reference with `label` as a prefix, then return the reference
    fn log_level_ref(&self, level: Level, label: &'static str) -> &Self {
        log::log!(level, "{label}: {:?}", self);
        self
    }

    /// log a `self` `mut` reference with `label` as a prefix, then return the reference
    fn log_level_mut(&mut self, level: Level, label: &'static str) -> &mut Self {
        log::log!(level, "{label}: {:?}", self);
        self
    }
}

impl<T> LogSelf for T where T: std::fmt::Debug {}
