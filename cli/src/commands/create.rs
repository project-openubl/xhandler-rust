use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::input::{self, DocumentInput};

#[derive(Args)]
pub struct CreateArgs {
    /// Archivo de entrada JSON/YAML. Usar "-" para leer desde stdin
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Ruta del archivo XML de salida. Si se omite, se imprime en stdout
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Formato de entrada cuando se lee desde stdin: json, yaml
    #[arg(long = "format")]
    pub format: Option<String>,

    /// Validar y enriquecer sin generar el XML
    #[arg(long)]
    pub dry_run: bool,
}

/// Creates UBL XML from an input file. Reads, enriches, and renders the document.
pub fn create_xml(input_file: &str, format: Option<&str>) -> anyhow::Result<String> {
    let doc = input::read_input(input_file, format)?;

    let defaults = Defaults {
        icb_tasa: rust_decimal_macros::dec!(0.2),
        igv_tasa: rust_decimal_macros::dec!(0.18),
        ivap_tasa: rust_decimal_macros::dec!(0.04),
        date: chrono::Local::now().date_naive(),
    };

    let xml = match doc {
        DocumentInput::Invoice { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::CreditNote { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::DebitNote { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::DespatchAdvice { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::Perception { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::Retention { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::SummaryDocuments { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
        DocumentInput::VoidedDocuments { mut spec } => {
            spec.enrich(&defaults);
            spec.render()?
        }
    };

    Ok(xml)
}

impl CreateArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        if self.dry_run {
            let doc = input::read_input(&self.input_file, self.format.as_deref())?;

            let defaults = Defaults {
                icb_tasa: rust_decimal_macros::dec!(0.2),
                igv_tasa: rust_decimal_macros::dec!(0.18),
                ivap_tasa: rust_decimal_macros::dec!(0.04),
                date: chrono::Local::now().date_naive(),
            };

            return match doc {
                DocumentInput::Invoice { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::CreditNote { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::DebitNote { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::DespatchAdvice { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::Perception { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::Retention { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::SummaryDocuments { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
                DocumentInput::VoidedDocuments { mut spec } => {
                    spec.enrich(&defaults);
                    self.print_dry_run(&spec)
                }
            };
        }

        let xml = create_xml(&self.input_file, self.format.as_deref())?;
        self.write_output(&xml)?;
        Ok(ExitCode::SUCCESS)
    }

    fn write_output(&self, xml: &str) -> anyhow::Result<()> {
        match &self.output_file {
            Some(path) => std::fs::write(path, xml)?,
            None => print!("{xml}"),
        }
        Ok(())
    }

    fn print_dry_run<T: serde::Serialize>(&self, spec: &T) -> anyhow::Result<ExitCode> {
        let json = serde_json::to_string_pretty(spec)?;
        eprintln!("{json}");
        Ok(ExitCode::SUCCESS)
    }
}
