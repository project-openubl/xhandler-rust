use std::io::Read;
use std::path::Path;

use anyhow::Context;
use xhandler::prelude::*;

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

    pub fn parse(&self, content: &str) -> anyhow::Result<Document> {
        match self {
            InputFormat::Json => serde_json::from_str(content).map_err(|e| {
                anyhow::anyhow!(
                    "Error en el archivo JSON:\n  {e}\n  Valores validos de 'kind': {}",
                    Document::KINDS
                )
            }),
            InputFormat::Yaml => serde_yaml::from_str(content).map_err(|e| {
                anyhow::anyhow!(
                    "Error en el archivo YAML:\n  {e}\n  Valores validos de 'kind': {}",
                    Document::KINDS
                )
            }),
        }
    }
}

pub fn read_input(file: &str, format: Option<&str>) -> anyhow::Result<Document> {
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
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("no se puede leer el archivo: {}", path.display()))?;
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
