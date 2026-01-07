use clap::Parser as _;
use env_logger::Logger;
use logging_options::Backend as _;

use crate::cliopts::Options;

pub fn run() {
    let opts = Options::parse();
    Logger::init_from_options(&opts.logopts);
    log::debug!("Logging initialized.");
}
