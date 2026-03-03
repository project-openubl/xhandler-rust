use rust_decimal_macros::dec;
use xbuilder::prelude::*;

mod common;

const BASE_INAFECTO: &str = "tests/resources/e2e/homologacion/Group9InafectoTest";
const BASE_EXONERADO: &str = "tests/resources/e2e/homologacion/Group9ExoneradoTest";

/// Item 1 is onerosa, items 2+ are gratuita (pipeline sets precio=0 and computes precio_referencia).
fn items_group9(
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
                descripcion: Box::leak(format!("Item{i}").into_boxed_str()),
                cantidad: dec!(1) * rust_decimal::Decimal::from(i),
                precio: Some(dec!(100) * rust_decimal::Decimal::from(i)),
                igv_tipo: Some(code),
                ..Default::default()
            }
        })
        .collect()
}

fn items_inafecto(n: usize) -> Vec<Detalle> {
    items_group9(
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
    items_group9(
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

    // Cases 63-67: Boletas

    #[serial_test::serial]
    #[tokio::test]
    async fn caso63_boleta1_con_2_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-1",
            detalles: items_inafecto(2),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/boleta1Con2Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso64_boleta2_con_4_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-2",
            detalles: items_inafecto(4),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/boleta2Con4Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso65_boleta3_con_7_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-3",
            detalles: items_inafecto(7),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/boleta3Con7Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso66_boleta4_con_5_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-4",
            detalles: items_inafecto(5),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/boleta4Con5Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso67_boleta5_con_1_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-5",
            detalles: items_inafecto(1),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_INAFECTO}/boleta5Con1Items.xml"),
        )
        .await;
    }

    // Cases 68-70: Notas de Credito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso68_nota_credito_boleta1() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-1",
            comprobante_afectado_serie_numero: "BB12-1",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(2),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeBoleta1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso69_nota_credito_boleta4() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-2",
            comprobante_afectado_serie_numero: "BB12-4",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(5),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeBoleta4.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso70_nota_credito_boleta5() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-3",
            comprobante_afectado_serie_numero: "BB12-5",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(1),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_INAFECTO}/notaDeCreditoDeBoleta5.xml"),
        )
        .await;
    }

    // Cases 71-73: Notas de Debito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso71_nota_debito_boleta1() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-1",
            comprobante_afectado_serie_numero: "BB12-1",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(2),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeBoleta1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso72_nota_debito_boleta4() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-2",
            comprobante_afectado_serie_numero: "BB12-4",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(5),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeBoleta4.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso73_nota_debito_boleta5() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-3",
            comprobante_afectado_serie_numero: "BB12-5",
            sustento_descripcion: "Homologacion",
            detalles: items_inafecto(1),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_INAFECTO}/notaDeDebitoDeBoleta5.xml"),
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

    // Cases 63-67: Boletas

    #[serial_test::serial]
    #[tokio::test]
    async fn caso63_boleta1_con_2_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-1",
            detalles: items_exonerado(2),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/boleta1Con2Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso64_boleta2_con_4_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-2",
            detalles: items_exonerado(4),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/boleta2Con4Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso65_boleta3_con_7_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-3",
            detalles: items_exonerado(7),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/boleta3Con7Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso66_boleta4_con_5_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-4",
            detalles: items_exonerado(5),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/boleta4Con5Items.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso67_boleta5_con_1_items() {
        let mut invoice = Invoice {
            serie_numero: "BB12-5",
            detalles: items_exonerado(1),
            ..invoice_base()
        };
        assert_invoice(
            &mut invoice,
            &format!("{BASE_EXONERADO}/boleta5Con1Items.xml"),
        )
        .await;
    }

    // Cases 68-70: Notas de Credito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso68_nota_credito_boleta1() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-1",
            comprobante_afectado_serie_numero: "BB12-1",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(2),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeBoleta1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso69_nota_credito_boleta4() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-2",
            comprobante_afectado_serie_numero: "BB12-4",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(5),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeBoleta4.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso70_nota_credito_boleta5() {
        let mut credit_note = CreditNote {
            serie_numero: "BB12-3",
            comprobante_afectado_serie_numero: "BB12-5",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(1),
            ..credit_note_base()
        };
        assert_credit_note(
            &mut credit_note,
            &format!("{BASE_EXONERADO}/notaDeCreditoDeBoleta5.xml"),
        )
        .await;
    }

    // Cases 71-73: Notas de Debito

    #[serial_test::serial]
    #[tokio::test]
    async fn caso71_nota_debito_boleta1() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-1",
            comprobante_afectado_serie_numero: "BB12-1",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(2),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeBoleta1.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso72_nota_debito_boleta4() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-2",
            comprobante_afectado_serie_numero: "BB12-4",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(5),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeBoleta4.xml"),
        )
        .await;
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn caso73_nota_debito_boleta5() {
        let mut debit_note = DebitNote {
            serie_numero: "BB12-3",
            comprobante_afectado_serie_numero: "BB12-5",
            sustento_descripcion: "Homologacion",
            detalles: items_exonerado(1),
            ..debit_note_base()
        };
        assert_debit_note(
            &mut debit_note,
            &format!("{BASE_EXONERADO}/notaDeDebitoDeBoleta5.xml"),
        )
        .await;
    }
}
