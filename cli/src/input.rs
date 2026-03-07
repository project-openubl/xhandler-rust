use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use xhandler::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum DocumentInput {
    Invoice { spec: Box<Invoice> },
    CreditNote { spec: Box<CreditNote> },
    DebitNote { spec: Box<DebitNote> },
    DespatchAdvice { spec: Box<DespatchAdvice> },
    Perception { spec: Box<Perception> },
    Retention { spec: Box<Retention> },
    SummaryDocuments { spec: Box<SummaryDocuments> },
    VoidedDocuments { spec: Box<VoidedDocuments> },
}

#[derive(Clone, Copy)]
pub enum InputFormat {
    Json,
    Yaml,
}

impl InputFormat {
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => Ok(InputFormat::Json),
            Some("yaml" | "yml") => Ok(InputFormat::Yaml),
            Some(ext) => {
                anyhow::bail!(
                    "extension de archivo no soportada: .{ext} (usar .json, .yaml o .yml)"
                )
            }
            None => anyhow::bail!(
                "no se puede detectar el formato: el archivo no tiene extension (usar --format)"
            ),
        }
    }

    pub fn parse(&self, content: &str) -> anyhow::Result<DocumentInput> {
        match self {
            InputFormat::Json => Ok(serde_json::from_str(content)?),
            InputFormat::Yaml => Ok(serde_yaml::from_str(content)?),
        }
    }
}

pub fn read_input(file: &str, format: Option<&str>) -> anyhow::Result<DocumentInput> {
    let (content, fmt) = if file == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        let fmt = match format {
            Some("json") => InputFormat::Json,
            Some("yaml") => InputFormat::Yaml,
            Some(f) => anyhow::bail!("formato no soportado: {f} (usar json o yaml)"),
            None => anyhow::bail!("se requiere --format cuando se lee desde stdin"),
        };
        (buf, fmt)
    } else {
        let path = Path::new(file);
        let content = std::fs::read_to_string(path)?;
        let fmt = match format {
            Some("json") => InputFormat::Json,
            Some("yaml") => InputFormat::Yaml,
            Some(f) => anyhow::bail!("formato no soportado: {f}"),
            None => InputFormat::from_path(path)?,
        };
        (content, fmt)
    };

    fmt.parse(&content)
}
