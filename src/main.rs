mod action;
mod app;
mod cli;
mod config;
mod errors;
mod logging;
mod serde_helper;
mod tui;
mod widgets;

use app::App;
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::parse();
    config::init(&cli)?;
    logging::init()?;
    errors::install_hooks()?;

    if cli.print_default_config {
        println!("{}", toml::to_string_pretty(config::get())?);
        return Ok(());
    }

    let tui = tui::Tui::new()?;
    App::new().run(tui).await?;

    Ok(())
}
