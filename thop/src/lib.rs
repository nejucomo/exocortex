//! [Thop] is the fundamental cognitive unit in `exocortex`
#![deny(unsafe_code, missing_docs)]

use derive_new::new;
use exocortex_timestamp::Timestamp;

/// A "thought on paper" is the fundamental cognitive unit in `exocortex`
#[derive(Debug, new)]
pub struct Thop {
    /// The creation time
    pub ctime: Timestamp,
    /// The most recent modification time
    pub mtime: Timestamp,
    /// A text of this [Thop]
    #[new(into)]
    pub synopsis: String,
}
