use crate::model::Action;
use crate::storage::Store;
use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "exocortex",
    about = "Micro-note taking app — run without subcommand to open the GUI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new jot
    Add {
        /// The text of the jot
        text: String,
    },
    /// Print the full action log
    Log,
    /// List jots, optionally filtered
    View {
        /// Case-insensitive substring search
        #[arg(long, short)]
        search: Option<String>,
        /// Only show jots created at or after this timestamp (RFC 3339)
        #[arg(long)]
        from: Option<String>,
        /// Only show jots created at or before this timestamp (RFC 3339)
        #[arg(long)]
        to: Option<String>,
    },
}

pub fn run(cmd: Commands) -> Result<()> {
    let store = Store::open()?;

    match cmd {
        Commands::Add { text } => {
            let action = Action::AddJot {
                timestamp: Utc::now(),
                id: Uuid::new_v4(),
                text,
            };
            store.append(&action)?;
            if let Action::AddJot { id, text, .. } = &action {
                println!("Added jot {} — {:?}", &id.to_string()[..8], text);
            }
        }

        Commands::Log => {
            let actions = store.load_actions()?;
            if actions.is_empty() {
                println!("(no actions yet)");
            }
            for action in &actions {
                println!(
                    "{}  {}",
                    action.timestamp().format("%Y-%m-%d %H:%M:%S UTC"),
                    action.description(),
                );
            }
        }

        Commands::View { search, from, to } => {
            let from_ts = from
                .as_deref()
                .map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .with_context(|| format!("invalid --from timestamp: {s}"))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                })
                .transpose()?;

            let to_ts = to
                .as_deref()
                .map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .with_context(|| format!("invalid --to timestamp: {s}"))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                })
                .transpose()?;

            let actions = store.load_actions()?;
            let jots = Store::compute_jots(&actions);
            let filtered =
                Store::filter_jots(&jots, search.as_deref(), from_ts, to_ts);

            if filtered.is_empty() {
                println!("(no jots match)");
            }
            for jot in filtered {
                println!(
                    "{}  {}  {}",
                    jot.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                    &jot.id.to_string()[..8],
                    jot.text,
                );
            }
        }
    }

    Ok(())
}
