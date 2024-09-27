use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod cli;
mod client;
mod common;
mod components;
mod config;
mod errors;
mod logging;
mod model;
mod tui;
mod widgets;

#[tokio::main]
async fn main() -> Result<()> {
    errors::init()?;
    logging::init()?;

    let args = Cli::parse();

    let client = client::Client::new()?;
    let model = client.initialize().await?;

    let mut app = App::new(args, model)?;
    app.run().await?;

    Ok(())
}
