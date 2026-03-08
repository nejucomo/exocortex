use clap::Parser as _;
use color_eyre::eyre::{Result, WrapErr, eyre};
use env_logger::Logger;
use exocortex_damo::MultiProvider;
use logging_options::Backend as _;

use crate::app::App;
use crate::cliopts::Options;
use crate::prepop::prepopulated;

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

    let damo = MultiProvider::open_or_create(opts.db_path.as_opt_path())
        .wrap_err_with(|| format!("{:?}", opts.db_path))?;

    let damo = prepopulated(damo)?;

    App::run(damo).or_else(|e| Err(eyre!("eframe error")).wrap_err_with(|| format!("{e}")))?;

    Ok(())
}
