use std::process::ExitCode;

use clap::Parser;

use openubl_cli::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let cli = Cli::parse();
    cli.command.run().await
}
