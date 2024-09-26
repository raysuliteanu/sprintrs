mod client;
mod common;
mod config;
mod model;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// App tick rate
    #[arg(short, long, default_value_t = 1000)]
    app_tick_rate: u64,
}

fn main() -> reqwest::Result<()> {
    let _cli = Cli::parse();
    let client = client::Client::new()?;
    client.initialize()?;

    Ok(())
}
