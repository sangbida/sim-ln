use clap::{Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(long, short)]
    config: PathBuf,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config_str = std::fs::read_to_string(cli.config)?;
    let config: serde_json::Value = serde_json::from_str(&config_str)?;

    println!("Config: {:?}", config);
    println!("Simulating...");
    println!("42 and Done!");

    Ok(())
}
