use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "openubl")]
#[command(about = "CLI para facturacion electronica SUNAT Peru: crear XML UBL, firmar y enviar")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
