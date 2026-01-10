use clap::Parser as _;
use env_logger::Logger;
use logging_options::Backend as _;

use crate::app::App;
use crate::cliopts::Options;

pub fn run() -> eframe::Result<()> {
    let opts = Options::parse();

    Logger::init_from_options(&opts.logopts);
    log::debug!("Logging initialized.");

    App::run()
}
