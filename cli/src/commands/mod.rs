use std::process::ExitCode;

use clap::Subcommand;

pub mod apply;
pub mod create;
pub mod send;
pub mod sign;
pub mod verify_ticket;

#[derive(Subcommand)]
pub enum Commands {
    /// Generate UBL XML from a JSON/YAML document definition
    Create(create::CreateArgs),

    /// Sign a UBL XML document with RSA-SHA256
    Sign(sign::SignArgs),

    /// Send a signed XML to SUNAT
    Send(send::SendArgs),

    /// Full pipeline: create + sign + send
    Apply(apply::ApplyArgs),

    /// Check status of an async submission (SummaryDocuments, VoidedDocuments)
    VerifyTicket(verify_ticket::VerifyTicketArgs),
}

impl Commands {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        match self {
            Commands::Create(args) => args.run().await,
            Commands::Sign(args) => args.run().await,
            Commands::Send(args) => args.run().await,
            Commands::Apply(args) => args.run().await,
            Commands::VerifyTicket(args) => args.run().await,
        }
    }
}
