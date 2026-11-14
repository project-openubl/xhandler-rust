use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::commands::send::{CredentialArgs, SendArgs};

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  xhandler verify-ticket --ticket 1703154974517 -o respuesta.zip --beta
  xhandler verify-ticket --ticket a7b3c1d2-0001 -o respuesta.zip --despatch --username 20123456789USUARIO --password clave --client-id id --client-secret secreto

La salida es JSON en stdout. Codigo de salida: 0=aceptado o en proceso, 2=rechazado o error SUNAT.")]
pub struct VerifyTicketArgs {
    /// Numero de ticket obtenido de un envio anterior
    #[arg(long)]
    pub ticket: String,

    /// Ruta del archivo CDR zip de respuesta
    #[arg(short = 'o', long)]
    pub output: String,

    #[command(flatten)]
    pub credentials: CredentialArgs,

    /// Endpoint SOAP de facturas SUNAT (usado para verificar tickets)
    #[arg(long, env = "XHANDLER_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// Endpoint REST de guias de remision SUNAT (con --despatch)
    #[arg(long, env = "XHANDLER_URL_DESPATCH")]
    pub url_despatch: Option<String>,

    /// El ticket es de una guia de remision (API REST, requiere --client-id y --client-secret)
    #[arg(long)]
    pub despatch: bool,

    /// Usar URLs de prueba y credenciales de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,
}

impl VerifyTicketArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let send_args = SendArgs {
            file: String::new(),
            output: Some(self.output.clone()),
            credentials: self.credentials.clone(),
            url_invoice: self.url_invoice.clone(),
            url_perception_retention: None,
            url_despatch: self.url_despatch.clone(),
            beta: self.beta,
            no_interactive: false,
        };

        if self.despatch {
            send_args.check_despatch_beta()?;
        }
        let urls = send_args.resolve_urls();
        let credentials = send_args.credentials.resolve(send_args.beta)?;

        let sender = FileSender { urls, credentials };
        let target = if self.despatch {
            VerifyTicketTarget::Rest(sender.urls.despatch.clone())
        } else {
            VerifyTicketTarget::Soap(sender.urls.invoice.clone())
        };

        let result = sender.verify_ticket(&target, &self.ticket).await?;
        let is_fault = matches!(result.response, VerifyTicketAggregatedResponse::Fault(_));
        let (output, status) = super::verify_ticket_output(result.response, &self.output)?;

        if is_fault {
            eprintln!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Ok(super::exit_code_for(status))
    }
}
