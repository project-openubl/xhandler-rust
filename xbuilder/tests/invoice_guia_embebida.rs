use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use crate::common::{assert_invoice, invoice_base};

mod common;

const BASE: &str = "tests/resources/e2e/renderer/invoice/InvoiceGuiaEmbebidaTest";

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
            llegada: direccion("150102", "Av. Destino 456"),
            transportista: Cliente {
                tipo_documento_identidad: "6".into(),
                numero_documento_identidad: "20123456789".into(),
                nombre: "Transportes Veloz S.A.C.".into(),
                ..Default::default()
            },
            numero_licencia: "Q12345678".into(),
            placa: "ABC-123".into(),
            modalidad_traslado: "01".into(),
            peso_bruto: dec!(10.5),
            unidad_peso_bruto: "KGM".into(),
        }),
        ..invoice_base()
    };

    assert_invoice(&mut invoice, &format!("{BASE}/guiaEmbebida.xml")).await;
}
