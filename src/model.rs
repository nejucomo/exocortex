use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a jot.
pub type JotId = Uuid;

/// A single micro-note (jot).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jot {
    pub id: JotId,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An immutable, append-only user-level action recorded in the log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    AddJot {
        timestamp: DateTime<Utc>,
        id: JotId,
        text: String,
    },
    EditJot {
        timestamp: DateTime<Utc>,
        id: JotId,
        new_text: String,
    },
}

impl Action {
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Action::AddJot { timestamp, .. } | Action::EditJot { timestamp, .. } => *timestamp,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Action::AddJot { text, .. } => format!("add jot: {:?}", text),
            Action::EditJot { id, new_text, .. } => {
                format!("edit jot {}: {:?}", &id.to_string()[..8], new_text)
            }
        }
    }
}
