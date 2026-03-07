use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "openubl")]
#[command(about = "CLI for Peru SUNAT electronic invoicing: create UBL XML, sign, and send")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
