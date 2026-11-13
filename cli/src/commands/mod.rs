use std::path::Path;
use std::process::ExitCode;

use clap::Subcommand;

pub mod apply;
pub mod create;
pub mod send;
pub mod sign;
pub mod verify_ticket;

#[derive(Subcommand)]
pub enum Commands {
    /// Genera XML UBL a partir de un archivo JSON/YAML
    Create(create::CreateArgs),

    /// Firma un documento XML UBL con RSA-SHA256
    Sign(sign::SignArgs),

    /// Envia un XML firmado a SUNAT
    Send(send::SendArgs),

    /// Pipeline completo: crear XML + firmar + enviar a SUNAT
    Apply(apply::ApplyArgs),

    /// Consulta el estado de un envio asincrono (Resumenes y Comunicaciones de baja)
    VerifyTicket(verify_ticket::VerifyTicketArgs),
}

/// Returns the absolute path for a given path string.
pub fn absolute_path(path: &str) -> String {
    let p = Path::new(path);
    if p.is_absolute() {
        return path.to_string();
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(p).to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

/// Exit code for a SUNAT outcome: 0 when the document was accepted (with or
/// without observations) or is still being processed, 2 otherwise.
pub fn exit_code_for(status: xhandler::prelude::Status) -> ExitCode {
    use xhandler::prelude::Status;
    match status {
        Status::Aceptado | Status::EnProceso => ExitCode::SUCCESS,
        Status::Rechazado | Status::Excepcion | Status::Unknown => ExitCode::from(2),
    }
}

/// Prompts the user to verify a ticket and runs verification if accepted.
/// Returns the verification JSON output and status, or None if the user declined.
pub async fn prompt_verify_ticket(
    ticket: &str,
    sender: &xhandler::prelude::FileSender,
    target: &xhandler::prelude::VerifyTicketTarget,
    default_cdr_path: &str,
) -> anyhow::Result<Option<(serde_json::Value, xhandler::prelude::Status)>> {
    use std::io::Write;

    eprint!("Ticket recibido: {ticket}\nDesea verificar el ticket ahora? [y/N]: ");
    std::io::stderr().flush()?;

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;

    if !answer.trim().eq_ignore_ascii_case("y") {
        return Ok(None);
    }

    let default_abs = absolute_path(default_cdr_path);
    eprint!("Ruta del archivo CDR [{default_abs}]: ");
    std::io::stderr().flush()?;

    let mut cdr_input = String::new();
    std::io::stdin().read_line(&mut cdr_input)?;
    let cdr_output = if cdr_input.trim().is_empty() {
        default_cdr_path.to_string()
    } else {
        cdr_input.trim().to_string()
    };

    let result = sender.verify_ticket(target, ticket).await?;
    verify_ticket_output(result.response, &cdr_output).map(Some)
}

/// Saves the CDR (if any) and builds the JSON output of a ticket verification.
pub fn verify_ticket_output(
    response: xhandler::prelude::VerifyTicketAggregatedResponse,
    cdr_output: &str,
) -> anyhow::Result<(serde_json::Value, xhandler::prelude::Status)> {
    use xhandler::prelude::{Status, VerifyTicketAggregatedResponse};

    match response {
        VerifyTicketAggregatedResponse::Cdr(status, metadata) => {
            use base64::Engine;
            let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&status.cdr_base64)?;
            std::fs::write(cdr_output, cdr_bytes)?;

            let cdr_status = metadata.status();
            Ok((
                serde_json::json!({
                    "cdr": absolute_path(cdr_output),
                    "status_code": status.status_code,
                    "response_code": metadata.response_code,
                    "status": cdr_status.to_string(),
                    "description": metadata.description,
                    "notes": metadata.notes,
                }),
                cdr_status,
            ))
        }
        VerifyTicketAggregatedResponse::NoCdr { status_code } => {
            let status = Status::from_code_str(&status_code);
            Ok((
                serde_json::json!({
                    "status_code": status_code,
                    "status": status.to_string(),
                }),
                status,
            ))
        }
        VerifyTicketAggregatedResponse::Error(error) => {
            let status = error.status();
            Ok((
                serde_json::json!({
                    "code": error.code,
                    "sunat_code": error.sunat_code(),
                    "status": status.to_string(),
                    "message": error.message,
                }),
                status,
            ))
        }
    }
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
