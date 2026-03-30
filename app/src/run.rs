use clap::Parser as _;
use color_eyre::eyre::{Result, WrapErr, eyre};
use env_logger::Logger;
use exocortex_db::Database;
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
    init_log(&opts.logopts);

    let db = Database::init(&opts.db_path).wrap_err_with(|| {
        format!(
            "Failed to initialize database in {:?}",
            opts.db_path.to_string()
        )
    })?;

    log::debug!("Launching db thread service...");
    let mut db = db.launch_thread_service();

    // FIXME: figure out how to avoid `e.to_string`
    stringify_error("db prepopulation error", prepopulate(&mut db))?;
    stringify_error("eframe error", App::run(db))?;

    Ok(())
}

fn init_log(logopts: &logging_options::StandardConsole) {
    use logging_options::backend::LoggingOptions as _;

    let mut b = Logger::builder();

    for noisymod in ["eframe", "egui", "egui_glow", "egui_winit"] {
        b.filter_module(noisymod, log::LevelFilter::Info);
    }

    logopts.configure(b).init();

    log::debug!("Logging initialized.");
}

fn stringify_error<E>(tag: &'static str, r: Result<(), E>) -> Result<()>
where
    E: ToString,
{
    r.or_else(|e| Err(eyre!(tag)).wrap_err_with(|| e.to_string()))
}
