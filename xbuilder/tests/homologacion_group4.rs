use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_credit_note;
use crate::common::assert_debit_note;
use crate::common::assert_invoice;
use crate::common::credit_note_base;
use crate::common::debit_note_base;
use crate::common::invoice_base;

mod common;

const BASE: &str = "tests/resources/e2e/homologacion/Group4Test";

/// Generates `n` items with default Gravado IGV (same as Group 1).
fn items(n: usize) -> Vec<Detalle> {
    (1..=n)
        .map(|i| Detalle {
            descripcion: format!("Item{i}"),
            cantidad: dec!(1) * rust_decimal::Decimal::from(i),
            precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
            ..Default::default()
        })
        .collect()
}

/// Factura 4 items: items 1-2 Gravado onerosa, item 3 GravadoRetiro (gratuita).
fn items_factura4() -> Vec<Detalle> {
    let mut detalles = items(2);
    detalles.push(Detalle {
        descripcion: "Item3".into(),
        cantidad: dec!(3),
        precio: Some(dec!(300)),
        igv_tipo: Some(Catalog7::GravadoRetiro.code().into()),
        ..Default::default()
    });
    detalles
}

fn descuento_global() -> Vec<Descuento> {
    vec![Descuento {
        tipo: Some(
            Catalog53::DescuentoGlobalNoAfectaBaseImponibleIgvIvap
                .code()
                .into(),
        ),
        monto: dec!(100),
        monto_base: None,
        factor: None,
    }]
}

// Cases 32-36: Facturas

#[serial_test::serial]
#[tokio::test]
async fn caso32_factura1_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "FF14-1".into(),
        detalles: items(2),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura1Con2Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso33_factura2_con_2_items() {
    let mut invoice = Invoice {
        serie_numero: "FF14-2".into(),
        detalles: items(2),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura2Con2Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso34_factura3_con_4_items() {
    let mut invoice = Invoice {
        serie_numero: "FF14-3".into(),
        detalles: items(4),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura3Con4Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso35_factura4_con_3_items() {
    let mut invoice = Invoice {
        serie_numero: "FF14-4".into(),
        detalles: items_factura4(),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura4Con3Items.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso36_factura5_con_5_items() {
    let mut invoice = Invoice {
        serie_numero: "FF14-5".into(),
        detalles: items(5),
        descuentos: descuento_global(),
        ..invoice_base()
    };
    assert_invoice(&mut invoice, &format!("{BASE}/factura5Con5Items.xml")).await;
}

// Cases 37-39: Notas de Credito

#[serial_test::serial]
#[tokio::test]
async fn caso37_nota_credito_factura2() {
    let mut credit_note = CreditNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(2),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso38_nota_credito_factura3() {
    let mut credit_note = CreditNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(3),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso39_nota_credito_factura5() {
    let mut credit_note = CreditNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-5".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..credit_note_base()
    };
    assert_credit_note(
        &mut credit_note,
        &format!("{BASE}/notaDeCreditoDeFactura5.xml"),
    )
    .await;
}

// Cases 40-42: Notas de Debito

#[serial_test::serial]
#[tokio::test]
async fn caso40_nota_debito_factura2() {
    let mut debit_note = DebitNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-2".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(2),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura2.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso41_nota_debito_factura3() {
    let mut debit_note = DebitNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-3".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(3),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura3.xml"),
    )
    .await;
}

#[serial_test::serial]
#[tokio::test]
async fn caso42_nota_debito_factura5() {
    let mut debit_note = DebitNote {
        serie_numero: "FF14-1".into(),
        comprobante_afectado_serie_numero: "FF14-5".into(),
        sustento_descripcion: "Homologacion".into(),
        detalles: items(5),
        ..debit_note_base()
    };
    assert_debit_note(
        &mut debit_note,
        &format!("{BASE}/notaDeDebitoDeFactura5.xml"),
    )
    .await;
}
