use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::input;

#[derive(Args)]
#[command(after_help = "\x1b[1mEjemplos:\x1b[0m
  xhandler create -f factura.yaml
  xhandler create -f factura.yaml -o factura.xml
  cat factura.json | xhandler create -f - --format json")]
pub struct CreateArgs {
    /// Archivo de entrada JSON/YAML. Usar "-" para leer desde stdin
    #[arg(short = 'f', long)]
    pub file: String,

    /// Ruta del archivo XML de salida. Si se omite, se imprime en stdout
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Formato de entrada cuando se lee desde stdin: json, yaml
    #[arg(long)]
    pub format: Option<String>,

    /// Validar y enriquecer sin generar el XML
    #[arg(long)]
    pub dry_run: bool,
}

/// Reads the document of an input file and enriches it.
fn read_enriched(file: &str, format: Option<&str>) -> anyhow::Result<Document> {
    let mut document = input::read_input(file, format)?;
    document.enrich(&Defaults {
        icb_tasa: rust_decimal_macros::dec!(0.2),
        igv_tasa: rust_decimal_macros::dec!(0.18),
        ivap_tasa: rust_decimal_macros::dec!(0.04),
        fecha_emision: chrono::Local::now().date_naive(),
    })?;
    Ok(document)
}

/// Creates UBL XML from an input file. Reads, enriches, and renders the document.
pub fn create_xml(file: &str, format: Option<&str>) -> anyhow::Result<String> {
    Ok(read_enriched(file, format)?.render()?)
}

impl CreateArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        if self.dry_run {
            let document = read_enriched(&self.file, self.format.as_deref())?;
            eprintln!("{}", serde_json::to_string_pretty(&document)?);
            return Ok(ExitCode::SUCCESS);
        }

        let xml = create_xml(&self.file, self.format.as_deref())?;
        self.write_output(&xml)?;
        Ok(ExitCode::SUCCESS)
    }

    fn write_output(&self, xml: &str) -> anyhow::Result<()> {
        match &self.output {
            Some(path) => std::fs::write(path, xml)?,
            None => print!("{xml}"),
        }
        Ok(())
    }
}
