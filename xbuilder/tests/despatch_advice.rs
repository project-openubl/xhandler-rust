use chrono::NaiveDate;
use rust_decimal_macros::dec;
use xbuilder::prelude::*;

use crate::common::assert_despatch_advice;

mod common;

const BASE: &str = "tests/resources/e2e/renderer/despatch_advice";

fn fecha() -> NaiveDate {
    NaiveDate::from_ymd_opt(2019, 12, 24).unwrap()
}

#[serial_test::serial]
#[tokio::test]
async fn despatch_advice_min_data() {
    let mut doc = DespatchAdvice {
        serie_numero: "T001-1".into(),
        tipo_comprobante: Some("09".into()),
        remitente: Remitente {
            ruc: "12345678912".into(),
            razon_social: "Softgreen S.A.C.".into(),
            ..Default::default()
        },
        destinatario: Destinatario {
            tipo_documento_identidad: "1".into(),
            numero_documento_identidad: "12345678".into(),
            nombre: "mi cliente".into(),
        },
        envio: Envio {
            tipo_traslado: "18".into(),
            peso_total: dec!(1),
            peso_total_unidad_medida: "KG".into(),
            tipo_modalidad_traslado: "02".into(),
            fecha_traslado: Some(fecha()),
            partida: Some(Partida {
                direccion: "DireccionOrigen".into(),
                ubigeo: "010101".into(),
                ..Default::default()
            }),
            destino: Some(Destino {
                direccion: "DireccionDestino".into(),
                ubigeo: "020202".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        detalles: vec![DespatchAdviceItem {
            cantidad: dec!(0.5),
            unidad_medida: "KG".into(),
            codigo: "123456".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_despatch_advice(&mut doc, &format!("{BASE}/min_data.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn despatch_advice_carrier_data() {
    let mut doc = DespatchAdvice {
        serie_numero: "V001-1".into(),
        version: Some("2.1".into()),
        tipo_comprobante: Some("31".into()),
        // In a "31" the remitente is the carrier issuing the guia
        remitente: Remitente {
            ruc: "20123456789".into(),
            razon_social: "Transportes Veloz S.A.C.".into(),
            numero_registro_mtc: Some("MTC-654321".into()),
        },
        destinatario: Destinatario {
            tipo_documento_identidad: "6".into(),
            numero_documento_identidad: "20876543210".into(),
            nombre: "Cliente Final S.A.".into(),
        },
        // The original remitente, who requested the transport
        tercero: Some(Tercero {
            tipo_documento_identidad: "6".into(),
            numero_documento_identidad: "20555555555".into(),
            nombre: "Empresa Vendedora S.A.C.".into(),
        }),
        envio: Envio {
            tipo_traslado: "01".into(),
            peso_total: dec!(500.00),
            peso_total_unidad_medida: "KGM".into(),
            tipo_modalidad_traslado: "01".into(),
            fecha_traslado: Some(fecha()),
            choferes: vec![Chofer {
                tipo_documento_identidad: "1".into(),
                numero_documento_identidad: "44444444".into(),
                nombres: Some("Carlos".into()),
                apellidos: Some("Guerrero".into()),
                licencia: Some("L5555555".into()),
                ..Default::default()
            }],
            vehiculo: Some(Vehiculo {
                placa: "XYZ-789".into(),
                numero_circulacion: Some("TUC-V001".into()),
                ..Default::default()
            }),
            partida: Some(Partida {
                direccion: "Almacen Principal".into(),
                ubigeo: "150101".into(),
                ..Default::default()
            }),
            destino: Some(Destino {
                direccion: "Tienda Centro".into(),
                ubigeo: "150102".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        detalles: vec![DespatchAdviceItem {
            cantidad: dec!(10.00),
            unidad_medida: "NIU".into(),
            codigo: "PROD-99".into(),
            descripcion: Some("Mercaderia variada".into()),
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_despatch_advice(&mut doc, &format!("{BASE}/carrier_data.xml")).await;
}

#[serial_test::serial]
#[tokio::test]
async fn despatch_advice_complex_data() {
    let cliente = Destinatario {
        tipo_documento_identidad: "1".into(),
        numero_documento_identidad: "12345678".into(),
        nombre: "Mi Cliente S.A.C.".into(),
    };

    let mut doc = DespatchAdvice {
        serie_numero: "T001-100".into(),
        version: Some("2.1".into()),
        tipo_comprobante: Some("09".into()),
        remitente: Remitente {
            ruc: "12345678912".into(),
            razon_social: "Softgreen S.A.C.".into(),
            ..Default::default()
        },
        destinatario: cliente.clone(),
        comprador: Some(cliente),
        documentos_adicionales: vec![DocumentoAdicional {
            tipo_documento: "09".into(),
            numero: "T001-1".into(),
            ruc_emisor: Some("20100010001".into()),
            tipo_documento_descripcion: Some("GUIA REMISION".into()),
        }],
        envio: Envio {
            tipo_traslado: "01".into(),
            peso_total: dec!(100.50),
            peso_total_unidad_medida: "KGM".into(),
            peso_items: Some(dec!(90.00)),
            sustento_peso: Some("Empaque madera".into()),
            numero_bultos: Some(10),
            tipo_modalidad_traslado: "01".into(),
            fecha_traslado: Some(fecha()),
            choferes: vec![
                Chofer {
                    tipo: Some("CONDUCTOR_PRINCIPAL".into()),
                    tipo_documento_identidad: "1".into(),
                    numero_documento_identidad: "11111111".into(),
                    nombres: Some("Juan".into()),
                    apellidos: Some("Perez".into()),
                    licencia: Some("Q1234567".into()),
                },
                Chofer {
                    tipo: Some("COPILOTO".into()),
                    tipo_documento_identidad: "1".into(),
                    numero_documento_identidad: "22222222".into(),
                    nombres: Some("Jose".into()),
                    apellidos: Some("Gomez".into()),
                    licencia: None,
                },
            ],
            contenedores: vec!["CONT-001".into(), "CONT-002".into()],
            vehiculo: Some(Vehiculo {
                placa: "ABC-123".into(),
                numero_circulacion: Some("TUC-001".into()),
                numero_autorizacion: Some("AUTH-001".into()),
                codigo_emisor: Some("MTC".into()),
                secundarios: vec![Vehiculo {
                    placa: "CAR-456".into(),
                    numero_circulacion: Some("TUC-SEC".into()),
                    ..Default::default()
                }],
            }),
            puerto: Some(Puerto {
                codigo: "CALLAO".into(),
                nombre: "Puerto del Callao".into(),
            }),
            partida: Some(Partida {
                direccion: "Av. Origen 123".into(),
                ubigeo: "010101".into(),
                codigo_local: Some("0001".into()),
                ruc: Some("12345678912".into()),
            }),
            destino: Some(Destino {
                direccion: "Av. Destino 456".into(),
                ubigeo: "020202".into(),
                codigo_local: Some("0002".into()),
                ruc: Some("87654321098".into()),
            }),
            ..Default::default()
        },
        detalles: vec![DespatchAdviceItem {
            cantidad: dec!(5.00),
            unidad_medida: "NIU".into(),
            codigo: "ITEM-01".into(),
            descripcion: Some("Caja de herramientas".into()),
            atributos: vec![
                DespatchAdviceAtributo {
                    nombre: "Color".into(),
                    codigo: "1001".into(),
                    valor: Some("Rojo".into()),
                },
                DespatchAdviceAtributo {
                    nombre: "Marca".into(),
                    codigo: "1002".into(),
                    valor: Some("ToolMaster".into()),
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_despatch_advice(&mut doc, &format!("{BASE}/complex_data.xml")).await;
}

#[test]
fn despatch_advice_requires_fecha_traslado() {
    let mut doc = DespatchAdvice {
        serie_numero: "T001-1".into(),
        envio: Envio {
            fecha_traslado: None,
            ..Default::default()
        },
        ..Default::default()
    };

    let error = doc
        .enrich(&common::defaults_base())
        .unwrap_err()
        .to_string();
    assert!(error.contains("envio.fecha_traslado"), "{error}");
}
