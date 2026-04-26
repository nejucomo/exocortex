//! DbRequest request type
use std::sync::Arc;

use derive_more::{From, Into, TryInto};

use crate::def_transitive_conversion::def_transitive_conversion;
use crate::modifications::{ThopCreate, ThopModify, ThopSetSynopsis};
use crate::queries::{Query, Scan, ScanNext, ScanQuery, ScanRelease, ThopCount};

/// A request to a [`Provider`](crate::Provider), wrapping a shared [`RequestInfo`]
#[derive(Clone, Debug, From, Into)]
#[from(Arc<RequestInfo>, RequestInfo)]
pub struct Request(Arc<RequestInfo>);

impl Request {
    /// Access the inner [`RequestInfo`]
    pub fn info(&self) -> &RequestInfo {
        self.0.as_ref()
    }
}

/// The top-level request sent by applications to the DB
#[derive(Debug, From, TryInto)]
pub enum RequestInfo {
    /// A read-only query
    #[from(Query, ScanQuery)]
    Query(Query),
    /// A write mutation
    #[from(ThopModify, ThopCreate, ThopSetSynopsis)]
    Modify(ThopModify),
}

def_transitive_conversion!(From: Query => RequestInfo => Request);
def_transitive_conversion!(From: ThopCount => Query => Request);
def_transitive_conversion!(From: ScanQuery => Query => Request);
def_transitive_conversion!(From: Scan => ScanQuery => Request);
def_transitive_conversion!(From: ScanNext => ScanQuery => Request);
def_transitive_conversion!(From: ScanRelease => ScanQuery => Request);

def_transitive_conversion!(From: ThopModify => RequestInfo => Request);
def_transitive_conversion!(From: ThopCreate => ThopModify => Request);
def_transitive_conversion!(From: ThopSetSynopsis => ThopModify => Request);
