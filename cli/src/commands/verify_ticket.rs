use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::commands::send::SendArgs;

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  openubl verify-ticket --ticket 1703154974517 -o respuesta.zip --beta

La salida es JSON en stdout. Codigo de salida: 0=exito, 2=error SUNAT.")]
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
        let target = VerifyTicketTarget::Soap(self.url_invoice.clone().unwrap_or_else(|| {
            if self.beta {
                "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string()
            } else {
                "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService".to_string()
            }
        }));

        let result = sender.verify_ticket(&target, &self.ticket).await?;

        match result.response {
            VerifyTicketAggregatedResponse::Cdr(status, metadata) => {
                use base64::Engine;
                let cdr_bytes =
                    base64::engine::general_purpose::STANDARD.decode(&status.cdr_base64)?;
                std::fs::write(&self.output_file, cdr_bytes)?;

                let output = serde_json::json!({
                    "cdr": super::absolute_path(&self.output_file),
                    "status_code": status.status_code,
                    "response_code": metadata.response_code,
                    "description": metadata.description,
                    "notes": metadata.notes,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::SUCCESS)
            }
            VerifyTicketAggregatedResponse::Error(error) => {
                let output = serde_json::json!({
                    "code": error.code,
                    "message": error.message,
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
                Ok(ExitCode::from(2))
            }
        }
    }
}
