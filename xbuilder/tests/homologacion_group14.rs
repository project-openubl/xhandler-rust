use chrono::NaiveDate;
use xbuilder::prelude::*;

use crate::common::assert_voided_documents;
use crate::common::proveedor_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group14Test";

fn voided_items() -> Vec<VoidedDocumentsItem> {
    (1..=5)
        .map(|i| VoidedDocumentsItem {
            serie: "F001".into(),
            numero: i,
            descripcion_sustento: format!("Motivo de baja {i}"),
            ..Default::default()
        })
        .collect()
}

// Case 100: Comunicacion de Baja con 5 items

#[serial_test::serial]
#[tokio::test]
async fn caso100_comunicacion_de_baja() {
    let mut doc = VoidedDocuments {
        numero: 1,
        fecha_emision_comprobantes: NaiveDate::from_ymd_opt(2019, 12, 22),
        proveedor: proveedor_base(),
        comprobantes: voided_items(),
        ..Default::default()
    };
    assert_voided_documents(&mut doc, &format!("{BASE}/comunicacionDeBaja.xml")).await;
}
