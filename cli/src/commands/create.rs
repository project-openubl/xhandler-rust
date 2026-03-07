use std::process::ExitCode;

use clap::Args;

use xhandler::prelude::*;

use crate::input::{self, DocumentInput};

#[derive(Args)]
pub struct CreateArgs {
    /// Input JSON/YAML file. Use "-" for stdin.
    #[arg(short = 'f', long = "file")]
    pub input_file: String,

    /// Output XML file path. Writes to stdout if omitted.
    #[arg(short = 'o', long = "output")]
    pub output_file: Option<String>,

    /// Input format when reading from stdin: json, yaml
    #[arg(long = "format")]
    pub format: Option<String>,

    /// Validate and enrich without rendering XML
    #[arg(long)]
    pub dry_run: bool,
}

impl CreateArgs {
    pub async fn run(&self) -> anyhow::Result<ExitCode> {
        let doc = input::read_input(&self.input_file, self.format.as_deref())?;

        let defaults = Defaults {
            icb_tasa: rust_decimal_macros::dec!(0.2),
            igv_tasa: rust_decimal_macros::dec!(0.18),
            ivap_tasa: rust_decimal_macros::dec!(0.04),
            date: chrono::Local::now().date_naive(),
        };

        let xml = match doc {
            DocumentInput::Invoice { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::CreditNote { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::DebitNote { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::DespatchAdvice { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::Perception { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::Retention { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::SummaryDocuments { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
            DocumentInput::VoidedDocuments { mut spec } => {
                spec.enrich(&defaults);
                if self.dry_run {
                    return self.print_dry_run(&spec);
                }
                spec.render()?
            }
        };

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
