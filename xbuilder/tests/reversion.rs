use chrono::NaiveDate;
use xbuilder::prelude::*;

use crate::common::{assert_reversion, proveedor_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/reversion/ReversionTest";

#[serial_test::serial]
#[tokio::test]
async fn reversion_with_perceptions() {
    let mut doc = Reversion(VoidedDocuments {
        numero: 1,
        fecha_emision: NaiveDate::from_ymd_opt(2022, 1, 31),
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2022, 1, 29),
        proveedor: proveedor_base(),
        comprobantes: vec![
            VoidedDocumentsItem {
                tipo_comprobante: Some(Catalog1::Percepcion.code().into()),
                serie: "P001".into(),
                numero: 1,
                descripcion_sustento: "Anulacion de percepcion por error en emision".into(),
            },
            VoidedDocumentsItem {
                tipo_comprobante: Some(Catalog1::Percepcion.code().into()),
                serie: "P001".into(),
                numero: 2,
                descripcion_sustento: "Anulacion de percepcion por duplicado".into(),
            },
        ],
        ..Default::default()
    });

    assert_reversion(&mut doc, &format!("{BASE}/reversion.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn reversion_with_retentions() {
    let mut doc = Reversion(VoidedDocuments {
        numero: 2,
        fecha_emision: NaiveDate::from_ymd_opt(2022, 1, 31),
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2022, 1, 29),
        proveedor: proveedor_base(),
        comprobantes: vec![VoidedDocumentsItem {
            // Filled from the "R" serie
            tipo_comprobante: None,
            serie: "R001".into(),
            numero: 1,
            descripcion_sustento: "Anulacion de retencion por error en calculo".into(),
        }],
        ..Default::default()
    });

    assert_reversion(&mut doc, &format!("{BASE}/reversion_retention.xml")).await;
}
