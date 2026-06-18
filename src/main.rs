mod app;
mod cli;
mod model;
mod storage;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli_args = cli::Cli::parse();

    match cli_args.command {
        Some(cmd) => cli::run(cmd),
        None => run_gui(),
    }
}

fn run_gui() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Exocortex")
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Exocortex",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("GUI error: {e}"))
}
