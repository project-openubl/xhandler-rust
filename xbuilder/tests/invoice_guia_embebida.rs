use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, invoice_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice_guia_embebida";

fn direccion(ubigeo: &str, direccion: &str) -> Direccion {
    Direccion {
        ubigeo: Some(ubigeo.into()),
        direccion: Some(direccion.into()),
        departamento: Some("Lima".into()),
        provincia: Some("Lima".into()),
        distrito: Some("Lima".into()),
        codigo_pais: Some("PE".into()),
        urbanizacion: None,
        codigo_local: None,
    }
}

#[serial_test::serial]
#[tokio::test]
async fn invoice_guia_embebida() {
    let mut invoice = Invoice {
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: Decimal::ONE,
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        guia_embebida: Some(GuiaEmbebida {
            partida: direccion("150101", "Av. Origen 123"),
            destino: direccion("150102", "Av. Destino 456"),
            transportista: Cliente {
                tipo_documento_identidad: "6".into(),
                numero_documento_identidad: "20123456789".into(),
                nombre: "Transportes Veloz S.A.C.".into(),
                ..Default::default()
            },
            licencia: "Q12345678".into(),
            placa: "ABC-123".into(),
            tipo_modalidad_traslado: "01".into(),
            peso_bruto: dec!(10.5),
            peso_bruto_unidad_medida: "KGM".into(),
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/guia_embebida.xml")).await;
}
