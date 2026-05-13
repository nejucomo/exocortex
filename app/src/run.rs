use clap::Parser as _;
use color_eyre::eyre::{Result, WrapErr, eyre};
use env_logger::Logger;
use exocortex_memory::Provider;
use exocortex_memory_ram::RamMem;
use exocortex_memory_redb::RedMem;
use logging_options::Backend as _;

use crate::app::App;
use crate::cliopts::{DbOption, Options};
use crate::tutorial;

/// Run the app
///
/// # Caution
///
/// This expects to control all global process initialization/cleanup in the same manner as a `main` function. It is written expecting basically a `fn main() -> Result { run() }` wrapper. The means it does things like initializing global logging, managing uncaught error reporting, etc...
pub fn run() -> Result<()> {
    color_eyre::install()?;

    let opts = Options::parse();
    init_log(&opts.logopts);

    match opts.db {
        DbOption::Ram => run_db(RamMem::new()),
        DbOption::Path(path) => {
            let db = RedMem::init(&path).wrap_err_with(|| {
                format!("Failed to initialize database in {:?}", path.display())
            })?;
            run_db(db)
        }
    }
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

fn run_db<P>(mut db: P) -> Result<()>
where
    P: Provider + Send + 'static,
{
    tutorial::prepopulate(&mut db).tag_err("db prepopulation error")?;
    App::run(db).tag_err("eframe error")?;
    Ok(())
}

trait TagErr {
    fn tag_err(self, tag: &'static str) -> Result<()>;
}

impl<E> TagErr for Result<(), E>
where
    E: ToString,
{
    fn tag_err(self, tag: &'static str) -> Result<()> {
        self.or_else(|e| Err(eyre!(tag)).wrap_err_with(|| e.to_string()))
    }
}
