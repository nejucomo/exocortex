//! DbRequest request type
use std::sync::Arc;

use derive_more::{From, Into, TryInto};

use crate::Thop;
use crate::def_transitive_conversion::def_transitive_conversion;
use crate::modifications::{ThopModify, ThopSetSynopsis};
use crate::queries::{Query, Scan, ScanNext, ScanQuery, ScanRelease, ThopCount};

#[derive(Debug, From, Into)]
#[from(Arc<RequestInfo>, RequestInfo)]
pub struct Request(Arc<RequestInfo>);

/// The top-level request sent by applications to the DB
#[derive(Debug, From, TryInto)]
pub enum RequestInfo {
    #[from(Query, ScanQuery)]
    Query(Query),
    #[from(ThopModify, Thop, ThopSetSynopsis)]
    Modify(ThopModify),
}

def_transitive_conversion!(From: Query => RequestInfo => Request);
def_transitive_conversion!(From: ThopCount => Query => Request);
def_transitive_conversion!(From: ScanQuery => Query => Request);
def_transitive_conversion!(From: Scan => ScanQuery => Request);
def_transitive_conversion!(From: ScanNext => ScanQuery => Request);
def_transitive_conversion!(From: ScanRelease => ScanQuery => Request);

def_transitive_conversion!(From: ThopModify => RequestInfo => Request);
def_transitive_conversion!(From: Thop => ThopModify => Request);
def_transitive_conversion!(From: ThopSetSynopsis => ThopModify => Request);
