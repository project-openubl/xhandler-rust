use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xhandler::xbuilder::models::common::Detalle;
use xhandler::xbuilder::prelude::*;

use crate::common::{assert_credit_note, credit_note_base};

const BASE: &str = "tests/resources/xbuilder/e2e/renderer/credit_note_forma_pago";

/// Tipo 13: corrects the amount owed and the installments of a factura al
/// crédito (R.S. 193-2020)
#[serial_test::serial]
#[tokio::test]
async fn credit_note_ajuste_montos_y_fechas_de_pago() {
    let mut credit_note = CreditNote {
        tipo_nota: Some("13".into()),
        descripcion_sustento: "Ajuste de cuotas".into(),
        detalles: vec![Detalle {
            descripcion: "Item1".into(),
            cantidad: dec!(1),
            precio: Some(dec!(0)),
            ..Default::default()
        }],
        forma_pago: Some(FormaPago {
            tipo: None,
            total: None,
            cuotas: vec![
                CuotaPago {
                    importe: dec!(500),
                    fecha_pago: NaiveDate::from_ymd_opt(2099, 1, 20).unwrap(),
                },
                CuotaPago {
                    importe: dec!(680),
                    fecha_pago: NaiveDate::from_ymd_opt(2099, 2, 20).unwrap(),
                },
            ],
        }),
        ..credit_note_base()
    };

    assert_credit_note(&mut credit_note, &format!("{BASE}/tipo_13.xml")).await;
}
