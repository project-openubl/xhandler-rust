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

impl Enrich for Document {
    fn enrich(&mut self, defaults: &Defaults) -> anyhow::Result<()> {
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
}

impl Renderer for Document {
    fn render(&self) -> tera::Result<String> {
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
    use crate::enricher::{Defaults, Enrich};
    use crate::renderer::Renderer;

    const SPEC: &str = r#"{
        "serie_numero": "F001-1",
        "proveedor": {"ruc": "12345678912", "razon_social": "Softgreen S.A.C."},
        "cliente": {
            "nombre": "Carlos Feria",
            "numero_documento_identidad": "12121212121",
            "tipo_documento_identidad": "6"
        },
        "detalles": [{"descripcion": "Item1", "cantidad": 10, "precio": 100}]
    }"#;

    fn defaults() -> Defaults {
        Defaults {
            icb_tasa: dec!(0.2),
            igv_tasa: dec!(0.18),
            ivap_tasa: dec!(0.04),
            fecha_emision: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        }
    }

    #[test]
    fn document_reads_kind_and_spec() {
        let json = format!(r#"{{"kind": "Invoice", "spec": {SPEC}}}"#);
        let mut document: Document = serde_json::from_str(&json).unwrap();
        document.enrich(&defaults()).unwrap();

        assert!(document
            .render()
            .unwrap()
            .contains("<cbc:ID>F001-1</cbc:ID>"));
    }

    #[test]
    fn document_reads_spec_before_kind() {
        let json = format!(r#"{{"spec": {SPEC}, "kind": "Invoice"}}"#);
        assert!(matches!(
            serde_json::from_str::<Document>(&json).unwrap(),
            Document::Invoice(_)
        ));
    }

    /// The enriched document can be stored as JSON and read back
    #[test]
    fn document_round_trips_through_json() {
        let json = format!(r#"{{"kind": "Invoice", "spec": {SPEC}}}"#);
        let mut document: Document = serde_json::from_str(&json).unwrap();
        document.enrich(&defaults()).unwrap();

        let stored = serde_json::to_string(&document).unwrap();
        assert!(stored.starts_with(r#"{"kind":"Invoice","spec":{"#));
        let read_back: Document = serde_json::from_str(&stored).unwrap();
        assert_eq!(read_back.render().unwrap(), document.render().unwrap());
    }

    /// Payment in full has no installments; `cuotas` can be left out
    #[test]
    fn document_reads_forma_pago_contado_without_cuotas() {
        let spec = SPEC.replace(
            r#""detalles""#,
            r#""forma_pago": {"tipo": "Contado"}, "detalles""#,
        );
        let json = format!(r#"{{"kind": "Invoice", "spec": {spec}}}"#);
        let mut document: Document = serde_json::from_str(&json).unwrap();
        document.enrich(&defaults()).unwrap();
        assert!(document.render().unwrap().contains("Contado"));
    }

    #[test]
    fn document_rejects_unknown_kinds() {
        let error = serde_json::from_str::<Document>(r#"{"kind": "Factura", "spec": {}}"#)
            .unwrap_err()
            .to_string();
        assert!(error.contains("expected one of `Invoice`"), "{error}");
    }
}
