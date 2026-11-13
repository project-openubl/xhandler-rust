use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::commands::send::SendArgs;

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  openubl verify-ticket --ticket 1703154974517 -o respuesta.zip --beta

La salida es JSON en stdout. Codigo de salida: 0=aceptado o en proceso, 2=rechazado o error SUNAT.")]
pub struct VerifyTicketArgs {
    /// Numero de ticket obtenido de un envio anterior
    #[arg(long)]
    pub ticket: String,

    /// Ruta del archivo CDR zip de respuesta
    #[arg(short = 'o', long = "output")]
    pub output_file: String,

    /// Usuario SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_USERNAME")]
    pub username: Option<String>,

    /// Clave SOL de SUNAT (con --beta se usan credenciales de prueba)
    #[arg(long, env = "OPENUBL_PASSWORD")]
    pub password: Option<String>,

    /// Endpoint SOAP de facturas SUNAT (usado para verificar tickets)
    #[arg(long = "url-invoice", env = "OPENUBL_URL_INVOICE")]
    pub url_invoice: Option<String>,

    /// Usar URLs de prueba y credenciales de prueba de SUNAT beta
    #[arg(long)]
    pub beta: bool,
}

impl VerifyTicketArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let send_args = SendArgs {
            input_file: String::new(),
            output_file: Some(self.output_file.clone()),
            username: self.username.clone(),
            password: self.password.clone(),
            url_invoice: self.url_invoice.clone(),
            url_perception_retention: None,
            url_despatch: None,
            beta: self.beta,
            no_interactive: false,
        };

        let urls = send_args.resolve_urls();
        let credentials = send_args.resolve_credentials()?;

        let sender = FileSender { urls, credentials };
        let target = VerifyTicketTarget::Soap(sender.urls.invoice.clone());

        let result = sender.verify_ticket(&target, &self.ticket).await?;
        let is_fault = matches!(result.response, VerifyTicketAggregatedResponse::Error(_));
        let (output, status) = super::verify_ticket_output(result.response, &self.output_file)?;

        if is_fault {
            eprintln!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Ok(super::exit_code_for(status))
    }
}
