use clap::{Parser, Subcommand};

mod openapi;

#[derive(Debug, Parser)]
pub struct Xtask {
    #[command(subcommand)]
    command: Command,
}

impl Xtask {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Openapi(command) => command.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Used to generate and/or validate the openapi spec
    Openapi(openapi::Openapi),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Xtask::parse().run().await
}
