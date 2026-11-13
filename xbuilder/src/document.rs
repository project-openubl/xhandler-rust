use serde::{Deserialize, Serialize};

use crate::enricher::{Defaults, Enrich};
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::despatch_advice::DespatchAdvice;
use crate::models::invoice::Invoice;
use crate::models::perception_retention::{Perception, Retention};
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::{Reversion, VoidedDocuments};
use crate::renderer::Renderer;

/// Any document xbuilder creates, tagged with its kind. In JSON/YAML:
/// `{"kind": "Invoice", "spec": {...}}`.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "spec")]
pub enum Document {
    Invoice(Box<Invoice>),
    CreditNote(Box<CreditNote>),
    DebitNote(Box<DebitNote>),
    DespatchAdvice(Box<DespatchAdvice>),
    Perception(Box<Perception>),
    Retention(Box<Retention>),
    SummaryDocuments(Box<SummaryDocuments>),
    VoidedDocuments(Box<VoidedDocuments>),
    Reversion(Box<Reversion>),
}

impl Document {
    /// Valid values of `kind`
    pub const KINDS: &'static str = "Invoice, CreditNote, DebitNote, DespatchAdvice, Perception, Retention, SummaryDocuments, VoidedDocuments, Reversion";

    /// See [`Enrich::enrich`]
    pub fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()> {
        match self {
            Self::Invoice(spec) => spec.enrich(defaults),
            Self::CreditNote(spec) => spec.enrich(defaults),
            Self::DebitNote(spec) => spec.enrich(defaults),
            Self::DespatchAdvice(spec) => spec.enrich(defaults),
            Self::Perception(spec) => spec.enrich(defaults),
            Self::Retention(spec) => spec.enrich(defaults),
            Self::SummaryDocuments(spec) => spec.enrich(defaults),
            Self::VoidedDocuments(spec) => spec.enrich(defaults),
            Self::Reversion(spec) => spec.enrich(defaults),
        }
    }

    /// UBL XML of the document
    pub fn render(&self) -> tera::Result<String> {
        match self {
            Self::Invoice(spec) => spec.render(),
            Self::CreditNote(spec) => spec.render(),
            Self::DebitNote(spec) => spec.render(),
            Self::DespatchAdvice(spec) => spec.render(),
            Self::Perception(spec) => spec.render(),
            Self::Retention(spec) => spec.render(),
            Self::SummaryDocuments(spec) => spec.render(),
            Self::VoidedDocuments(spec) => spec.render(),
            Self::Reversion(spec) => spec.render(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    use super::Document;
    use crate::enricher::Defaults;

    #[test]
    fn document_reads_kind_and_spec() {
        let json = r#"{
            "kind": "Invoice",
            "spec": {
                "serie_numero": "F001-1",
                "proveedor": {"ruc": "12345678912", "razon_social": "Softgreen S.A.C."},
                "cliente": {
                    "nombre": "Carlos Feria",
                    "numero_documento_identidad": "12121212121",
                    "tipo_documento_identidad": "6"
                },
                "detalles": [{"descripcion": "Item1", "cantidad": 10, "precio": 100}]
            }
        }"#;
        let mut document: Document = serde_json::from_str(json).unwrap();
        let defaults = Defaults {
            icb_tasa: dec!(0.2),
            igv_tasa: dec!(0.18),
            ivap_tasa: dec!(0.04),
            fecha_emision: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        };
        document.enrich(&defaults).unwrap();

        let xml = document.render().unwrap();
        assert!(xml.contains("<cbc:ID>F001-1</cbc:ID>"));
        assert!(serde_json::to_string(&document)
            .unwrap()
            .starts_with(r#"{"kind":"Invoice","spec":{"#));
    }
}
