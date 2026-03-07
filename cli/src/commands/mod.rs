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

/// Prompts the user to verify a ticket and runs verification if accepted.
/// Returns the JSON output from verification, or None if the user declined.
pub async fn prompt_verify_ticket(
    ticket: &str,
    sender: &xhandler::prelude::FileSender,
    beta: bool,
    default_cdr_path: &str,
) -> anyhow::Result<Option<serde_json::Value>> {
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

    let target = xhandler::prelude::VerifyTicketTarget::Soap(if beta {
        "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string()
    } else {
        "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService".to_string()
    });

    let result = sender.verify_ticket(&target, ticket).await?;

    match result.response {
        xhandler::prelude::VerifyTicketAggregatedResponse::Cdr(status, metadata) => {
            use base64::Engine;
            let cdr_bytes = base64::engine::general_purpose::STANDARD.decode(&status.cdr_base64)?;
            std::fs::write(&cdr_output, cdr_bytes)?;

            Ok(Some(serde_json::json!({
                "cdr": absolute_path(&cdr_output),
                "status_code": status.status_code,
                "response_code": metadata.response_code,
                "description": metadata.description,
                "notes": metadata.notes,
            })))
        }
        xhandler::prelude::VerifyTicketAggregatedResponse::Error(error) => {
            Ok(Some(serde_json::json!({
                "code": error.code,
                "message": error.message,
            })))
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
