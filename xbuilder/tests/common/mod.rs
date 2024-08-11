use std::collections::HashMap;
use std::fs;

use chrono::NaiveDate;
use libxml::tree::Document;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use xbuilder::prelude::*;

use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;
use xsender::prelude::Credentials;
use xsender::prelude::DocumentType;
use xsender::prelude::FileSender;
use xsender::prelude::SendFileAggregatedResponse;
use xsender::prelude::UblFile;
use xsender::prelude::Urls;
use xsigner::RsaKeyPair;
use xsigner::XSigner;

const INVOICE_XSD: &str = "tests/resources/xsd/2.1/maindoc/UBL-Invoice-2.1.xsd";
const CREDIT_NOTE_XSD: &str = "tests/resources/xsd/2.1/maindoc/UBL-CreditNote-2.1.xsd";
const DEBIT_NOTE_XSD: &str = "tests/resources/xsd/2.1/maindoc/UBL-DebitNote-2.1.xsd";
const _DESPATCH_ADVICE_XSD: &str = "tests/resources/xsd/2.1/maindoc/UBL-DespatchAdvice-2.1.xsd";
const _VOIDED_DOCUMENTS_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-VoidedDocuments-1.0.xsd";
const _SUMMARY_DOCUMENTS_XSD: &str =
    "tests/resources/xsd/2.0/maindoc/UBLPE-SummaryDocuments-1.0.xsd";
const _PERCEPTION_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-Perception-1.0.xsd";
const _RETENTION_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-Retention-1.0.xsd";

lazy_static::lazy_static! {
    pub static ref CLIENT: FileSender = FileSender {
        urls: Urls {
            invoice: "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string(),
            perception_retention:"https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService".to_string(),
            despatch: "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem".to_string(),
        },
        credentials: Credentials {
            username: "12345678959MODDATOS".to_string(),
            password: "MODDATOS".to_string(),
        },
    };
}

pub fn defaults_base() -> Defaults {
    Defaults {
        date: NaiveDate::from_ymd_opt(2019, 12, 24).unwrap(),
        icb_tasa: dec!(0.2),
        igv_tasa: dec!(0.18),
        ivap_tasa: dec!(0.04),
    }
}

#[allow(dead_code)]
pub fn invoice_base() -> Invoice {
    Invoice {
        leyendas: HashMap::new(),

        serie_numero: "F001-1",
        tipo_comprobante: None,
        tipo_operacion: None,

        igv_tasa: None,
        icb_tasa: None,
        ivap_tasa: None,

        moneda: None,
        fecha_emision: None,
        hora_emision: None,
        fecha_vencimiento: None,
        forma_de_pago: None,
        direccion_entrega: None,
        observaciones: None,

        total_impuestos: None,
        total_importe: None,

        firmante: None,
        proveedor: proveedor_base(),
        cliente: cliente_base(),

        percepcion: None,
        detraccion: None,

        anticipos: vec![],
        descuentos: vec![],
        detalles: vec![],

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
    }
}

#[allow(dead_code)]
pub fn credit_note_base() -> CreditNote {
    CreditNote {
        leyendas: HashMap::new(),

        serie_numero: "FC01-1",
        tipo_nota: None,

        comprobante_afectado_serie_numero: "F001-1",
        comprobante_afectado_tipo: None,
        sustento_descripcion: "mi sustento",

        igv_tasa: None,
        icb_tasa: None,
        ivap_tasa: None,

        moneda: None,
        fecha_emision: None,

        total_impuestos: None,
        total_importe: None,

        firmante: None,
        proveedor: proveedor_base(),
        cliente: cliente_base(),

        detalles: vec![],

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
    }
}

#[allow(dead_code)]
pub fn debit_note_base() -> DebitNote {
    DebitNote {
        leyendas: HashMap::new(),

        serie_numero: "FD01-1",
        tipo_nota: None,

        comprobante_afectado_serie_numero: "F001-1",
        comprobante_afectado_tipo: None,
        sustento_descripcion: "mi sustento",

        igv_tasa: None,
        icb_tasa: None,
        ivap_tasa: None,

        moneda: None,
        fecha_emision: None,

        total_impuestos: None,
        total_importe: None,

        firmante: None,
        proveedor: proveedor_base(),
        cliente: cliente_base(),

        detalles: vec![],

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
    }
}

pub fn proveedor_base() -> Proveedor {
    Proveedor {
        ruc: "12345678912",
        razon_social: "Softgreen S.A.C.",
        nombre_comercial: None,
        direccion: None,
        contacto: None,
    }
}

pub fn cliente_base() -> Cliente {
    Cliente {
        tipo_documento_identidad: Catalog6::RUC.code(),
        numero_documento_identidad: "12121212121",
        nombre: "Carlos Feria",
        direccion: None,
        contacto: None,
    }
}

pub fn detalle_base() -> Detalle {
    Detalle {
        descripcion: "Item",
        cantidad: Decimal::ZERO,
        unidad_medida: None,

        precio: None,
        precio_con_impuestos: None,
        precio_referencia: None,
        precio_referencia_tipo: None,

        igv_tasa: None,
        icb_tasa: None,
        isc_tasa: None,

        igv_tipo: None,
        isc_tipo: None,

        icb_aplica: false,
        icb: None,

        igv: None,
        igv_base_imponible: None,

        isc: None,
        isc_base_imponible: None,

        total_impuestos: None,
    }
}

fn sign_xml(xml: &str) -> Document {
    let private_key_from_file = fs::read_to_string("tests/resources/certificates/private.key")
        .expect("Could not read private.key");
    let certificate_from_file = fs::read_to_string("tests/resources/certificates/public.cer")
        .expect("Could not read public.cer");

    let rsa_key_pair =
        RsaKeyPair::from_pkcs1_pem_and_certificate(&private_key_from_file, &certificate_from_file)
            .expect("Could not initialize RsaKeyPair");

    let signer = XSigner::from_string(xml).expect("Could parse xml");
    signer.sign(&rsa_key_pair).expect("Could not sign document");

    signer.xml_document
}

#[allow(dead_code)]
pub async fn assert_invoice(invoice: &mut Invoice, snapshot_filename: &str) {
    let defaults = defaults_base();
    invoice.enrich(&defaults);

    let xml = invoice.render().expect("Could not render invoice");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, INVOICE_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_credit_note(credit_note: &mut CreditNote, snapshot_filename: &str) {
    let defaults = defaults_base();
    credit_note.enrich(&defaults);

    let xml = credit_note.render().expect("Could not render credit note");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, CREDIT_NOTE_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_debit_note(debit_note: &mut DebitNote, snapshot_filename: &str) {
    let defaults = defaults_base();
    debit_note.enrich(&defaults);

    let xml = debit_note.render().expect("Could not render debit note");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, DEBIT_NOTE_XSD);
    assert_sunat(&xml_signed).await;
}

fn assert_snapshot(expected: &str, snapshot_filename: &str) {
    let snapshot_file_content = fs::read_to_string(snapshot_filename);
    assert!(snapshot_file_content.is_ok());

    assert_eq!(
        expected,
        snapshot_file_content.unwrap(),
        "File {} does not match",
        snapshot_filename
    );
}

fn assert_xsd(xml: &Document, schema: &str) {
    let mut xsdparser = SchemaParserContext::from_file(schema);
    let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

    if let Err(errors) = xsd {
        for err in &errors {
            println!("{}", err.message.as_ref().unwrap());
        }

        panic!("Failed to parse schema");
    }

    let mut xsd = xsd.unwrap();

    if let Err(errors) = xsd.validate_document(xml) {
        for err in &errors {
            println!("{}", err.message.as_ref().unwrap());
        }

        panic!("Invalid XML accoding to XSD schema");
    }
}

async fn assert_sunat(xml: &Document) {
    let xml_file = UblFile {
        file_content: xml.to_string(),
    };

    let result = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    let documet_type = xml_file
        .metadata()
        .expect("Could not extract xml metadata")
        .document_type;

    match documet_type.as_str() {
        DocumentType::VOIDED_DOCUMENTS | DocumentType::SUMMARY_DOCUMENTS => {
            let result = match result.response {
                SendFileAggregatedResponse::Cdr(_, _) => false,
                SendFileAggregatedResponse::Ticket(_) => true,
                SendFileAggregatedResponse::Error(_) => false,
            };
            assert!(result)
        }
        _ => {
            let result = match result.response {
                SendFileAggregatedResponse::Cdr(_, cdr_metadata) => {
                    assert_eq!("0", cdr_metadata.response_code);
                    assert_eq!(Vec::<String>::new(), cdr_metadata.notes);
                    true
                }
                SendFileAggregatedResponse::Ticket(_) => false,
                SendFileAggregatedResponse::Error(_) => false,
            };
            assert!(result)
        }
    };
}
