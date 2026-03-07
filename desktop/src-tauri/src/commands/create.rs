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

#[tauri::command]
pub fn create_xml(document_json: String) -> Result<String, String> {
    let doc: DocumentInput =
        serde_json::from_str(&document_json).map_err(|e| format!("Error en el JSON: {e}"))?;

    let defaults = Defaults {
        icb_tasa: rust_decimal_macros::dec!(0.2),
        igv_tasa: rust_decimal_macros::dec!(0.18),
        ivap_tasa: rust_decimal_macros::dec!(0.04),
        date: chrono::Local::now().date_naive(),
    };

    let xml = match doc {
        DocumentInput::Invoice { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::CreditNote { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::DebitNote { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::DespatchAdvice { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::Perception { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::Retention { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::SummaryDocuments { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
        DocumentInput::VoidedDocuments { mut spec } => {
            spec.enrich(&defaults);
            spec.render().map_err(|e| e.to_string())?
        }
    };

    Ok(xml)
}
