use rust_decimal_macros::dec;
use xbuilder::prelude::*;

mod common;

const BASE_INAFECTO: &str = "tests/resources/e2e/homologacion/Group2InafectoTest";
const BASE_EXONERADO: &str = "tests/resources/e2e/homologacion/Group2ExoneradoTest";

/// Generates `n` items for Group 2 tests.
/// Item 1 is onerosa, items 2+ are gratuita (pipeline sets precio=0 and computes precio_referencia).
fn items_group2(
    n: usize,
    onerosa_code: &'static str,
    gratuita_codes: &[&'static str],
) -> Vec<Detalle> {
    (1..=n)
        .map(|i| {
            let code = if i == 1 {
                onerosa_code
            } else {
                gratuita_codes[(i - 2) % gratuita_codes.len()]
            };
            Detalle {
                descripcion: format!("Item{i}"),
                cantidad: dec!(1) * rust_decimal::Decimal::from(i),
                precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
                igv_tipo: Some(code.into()),
                ..Default::default()
            }
        })
        .collect()
}

fn items_inafecto(n: usize) -> Vec<Detalle> {
    items_group2(
        n,
        Catalog7::InafectoOperacionOnerosa.code(),
        &[
            Catalog7::InafectoRetiroPorBonificacion.code(),
            Catalog7::InafectoRetiro.code(),
            Catalog7::InafectoRetiroPorMuestrasMedicas.code(),
            Catalog7::InafectoRetiroPorConvenioColectivo.code(),
            Catalog7::InafectoRetiroPorPremio.code(),
            Catalog7::InafectoRetiroPorPublicidad.code(),
        ],
    )
}

fn items_exonerado(n: usize) -> Vec<Detalle> {
    items_group2(
        n,
        Catalog7::ExoneradoOperacionOnerosa.code(),
        &[
            Catalog7::ExoneradoTransferenciaGratuita.code(),
            Catalog7::InafectoRetiro.code(),
            Catalog7::InafectoRetiroPorMuestrasMedicas.code(),
            Catalog7::InafectoRetiroPorConvenioColectivo.code(),
            Catalog7::InafectoRetiroPorPremio.code(),
            Catalog7::InafectoRetiroPorPublicidad.code(),
        ],
    )
}

#[cfg(test)]
mod inafecto {
    use super::*;
    use crate::common::{
        assert_credit_note, assert_debit_note, assert_invoice, credit_note_base, debit_note_base,
        invoice_base,
    };

    // Cases 12-16: Facturas

    #[serial_test::serial]
    #[tokio::test]
    async fn caso12_factura1_con_1_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-1".into(),
            detalles: items_inafecto(1),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/factura1Con1Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso13_factura2_con_4_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-2".into(),
            detalles: items_inafecto(4),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/factura2Con4Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso14_factura3_con_7_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-3".into(),
            detalles: items_inafecto(7),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/factura3Con7Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso15_factura4_con_5_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-4".into(),
            detalles: items_inafecto(5),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/factura4Con5Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso16_factura5_con_6_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-5".into(),
            detalles: items_inafecto(6),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/factura5Con6Items.xml"),
        )
        .await;
    }

    // Cases 17-19: Notas de Credito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso17_nota_credito_factura1() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-1".into(),
            comprobante_afectado_serie_numero: "FF12-1".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(1),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeFactura1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso18_nota_credito_factura3() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-2".into(),
            comprobante_afectado_serie_numero: "FF12-3".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(7),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeFactura3.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso19_nota_credito_factura5() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-3".into(),
            comprobante_afectado_serie_numero: "FF12-5".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(5),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeFactura5.xml"),
        )
        .await;
    }

    // Cases 20-22: Notas de Debito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso20_nota_debito_factura1() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-1".into(),
            comprobante_afectado_serie_numero: "FF12-1".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(1),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeFactura1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso21_nota_debito_factura3() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-2".into(),
            comprobante_afectado_serie_numero: "FF12-3".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(7),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeFactura3.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso22_nota_debito_factura5() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-3".into(),
            comprobante_afectado_serie_numero: "FF12-5".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_inafecto(6),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeFactura5.xml"),
        )
        .await;
    }
}

#[cfg(test)]
mod exonerado {
    use super::*;
    use crate::common::{
        assert_credit_note, assert_debit_note, assert_invoice, credit_note_base, debit_note_base,
        invoice_base,
    };

    // Cases 12-16: Facturas

    #[serial_test::serial]
    #[tokio::test]
    async fn caso12_factura1_con_1_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-1".into(),
            detalles: items_exonerado(1),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/factura1Con1Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso13_factura2_con_4_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-2".into(),
            detalles: items_exonerado(4),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/factura2Con4Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso14_factura3_con_7_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-3".into(),
            detalles: items_exonerado(7),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/factura3Con7Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso15_factura4_con_5_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-4".into(),
            detalles: items_exonerado(5),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/factura4Con5Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso16_factura5_con_6_items() {
        let mut invoice = Invoice {
            serie_numero: "FF12-5".into(),
            detalles: items_exonerado(6),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/factura5Con6Items.xml"),
        )
        .await;
    }

    // Cases 17-19: Notas de Credito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso17_nota_credito_factura1() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-1".into(),
            comprobante_afectado_serie_numero: "FF12-1".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(1),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeFactura1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso18_nota_credito_factura3() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-2".into(),
            comprobante_afectado_serie_numero: "FF12-3".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(7),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeFactura3.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso19_nota_credito_factura5() {
        let mut credit_note = CreditNote {
            serie_numero: "FF12-3".into(),
            comprobante_afectado_serie_numero: "FF12-5".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(5),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeFactura5.xml"),
        )
        .await;
    }

    // Cases 20-22: Notas de Debito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso20_nota_debito_factura1() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-1".into(),
            comprobante_afectado_serie_numero: "FF12-1".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(1),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeFactura1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso21_nota_debito_factura3() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-2".into(),
            comprobante_afectado_serie_numero: "FF12-3".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(7),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeFactura3.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso22_nota_debito_factura5() {
        let mut debit_note = DebitNote {
            serie_numero: "FF12-3".into(),
            comprobante_afectado_serie_numero: "FF12-5".into(),
            sustento_descripcion: "Homologacion".into(),
            detalles: items_exonerado(6),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeFactura5.xml"),
        )
        .await;
    }
}
