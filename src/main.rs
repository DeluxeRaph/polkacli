use polkacli::{run_command, Cli, Result};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run_command(cli.command).await
}

