use clap::Parser as _;
use color_eyre::eyre::{Result, WrapErr, eyre};
use env_logger::Logger;
use exocortex_redb::ExoDb;
use logging_options::Backend as _;

use crate::app::App;
use crate::cliopts::Options;
use crate::prepop::prepopulate;

/// Run the app
///
/// # Caution
///
/// This expects to control all global process initialization/cleanup in the same manner as a `main` function. It is written expecting basically a `fn main() -> Result { run() }` wrapper. The means it does things like initializing global logging, managing uncaught error reporting, etc...
pub fn run() -> Result<()> {
    color_eyre::install()?;

    let opts = Options::parse();

    Logger::init_from_options(&opts.logopts);
    log::debug!("Logging initialized.");

    let mut db = ExoDb::init(&opts.db_path).wrap_err_with(|| {
        format!(
            "Failed to initialize database in {:?}",
            opts.db_path.to_string()
        )
    })?;

    // FIXME: figure out how to avoid `e.to_string`
    stringify_error("db prepopulation error", prepopulate(&mut db))?;
    stringify_error("eframe error", App::run(db))?;

    Ok(())
}

fn stringify_error<E>(tag: &'static str, r: Result<(), E>) -> Result<()>
where
    E: ToString,
{
    r.or_else(|e| Err(eyre!(tag)).wrap_err_with(|| e.to_string()))
}
