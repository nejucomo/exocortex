use crate::model::{Action, Jot, JotId};
use anyhow::{Context, Result};
use chrono::DateTime;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Persistent store backed by an append-only JSONL action log.
///
/// The log is the source of truth. All views of jots are computed by
/// replaying the log from the beginning.
pub struct Store {
    log_path: PathBuf,
}

impl Store {
    /// Open (or create) the store in the platform data directory.
    pub fn open() -> Result<Self> {
        let data_dir = directories::ProjectDirs::from("", "", "exocortex")
            .context("could not locate platform data directory")?
            .data_dir()
            .to_path_buf();
        fs::create_dir_all(&data_dir).context("could not create data directory")?;
        let log_path = data_dir.join("log.jsonl");
        Ok(Self { log_path })
    }

    /// Open a store rooted at an explicit path (useful for testing).
    pub fn open_at(dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&dir)?;
        Ok(Self {
            log_path: dir.join("log.jsonl"),
        })
    }

    /// Append a single action to the on-disk log.
    pub fn append(&self, action: &Action) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .context("could not open log file")?;
        let line = serde_json::to_string(action).context("could not serialise action")?;
        writeln!(file, "{line}").context("could not write to log file")?;
        Ok(())
    }

    /// Load all actions from the on-disk log.
    pub fn load_actions(&self) -> Result<Vec<Action>> {
        if !self.log_path.exists() {
            return Ok(vec![]);
        }
        let file = File::open(&self.log_path).context("could not open log file")?;
        let reader = BufReader::new(file);
        let mut actions = Vec::new();
        for (lineno, line) in reader.lines().enumerate() {
            let line = line.context("error reading log file")?;
            if line.trim().is_empty() {
                continue;
            }
            let action: Action = serde_json::from_str(&line)
                .with_context(|| format!("malformed log entry at line {}", lineno + 1))?;
            actions.push(action);
        }
        Ok(actions)
    }

    /// Replay the action log and produce the current set of jots in
    /// insertion order.
    pub fn compute_jots(actions: &[Action]) -> Vec<Jot> {
        let mut map: HashMap<JotId, Jot> = HashMap::new();
        let mut order: Vec<JotId> = Vec::new();

        for action in actions {
            match action {
                Action::AddJot {
                    timestamp,
                    id,
                    text,
                } => {
                    map.insert(
                        *id,
                        Jot {
                            id: *id,
                            text: text.clone(),
                            created_at: *timestamp,
                            updated_at: *timestamp,
                        },
                    );
                    order.push(*id);
                }
                Action::EditJot {
                    timestamp,
                    id,
                    new_text,
                } => {
                    if let Some(jot) = map.get_mut(id) {
                        jot.text = new_text.clone();
                        jot.updated_at = *timestamp;
                    }
                }
            }
        }

        order
            .into_iter()
            .filter_map(|id| map.get(&id).cloned())
            .collect()
    }

    /// Filter jots by optional case-insensitive substring and/or timestamp range.
    pub fn filter_jots<'a>(
        jots: &'a [Jot],
        search: Option<&str>,
        from: Option<DateTime<chrono::Utc>>,
        to: Option<DateTime<chrono::Utc>>,
    ) -> Vec<&'a Jot> {
        jots.iter()
            .filter(|j| {
                if let Some(s) = search {
                    if !j.text.to_lowercase().contains(&s.to_lowercase()) {
                        return false;
                    }
                }
                if let Some(f) = from {
                    if j.created_at < f {
                        return false;
                    }
                }
                if let Some(t) = to {
                    if j.created_at > t {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
