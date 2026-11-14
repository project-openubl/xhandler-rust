use std::fs;

use chrono::NaiveDate;
use libxml::parser::Parser;

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
const DESPATCH_ADVICE_XSD: &str = "tests/resources/xsd/2.1/maindoc/UBL-DespatchAdvice-2.1.xsd";
const VOIDED_DOCUMENTS_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-VoidedDocuments-1.0.xsd";
const SUMMARY_DOCUMENTS_XSD: &str =
    "tests/resources/xsd/2.0/maindoc/UBLPE-SummaryDocuments-1.0.xsd";
const PERCEPTION_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-Perception-1.0.xsd";
const RETENTION_XSD: &str = "tests/resources/xsd/2.0/maindoc/UBLPE-Retention-1.0.xsd";

lazy_static::lazy_static! {
    pub static ref CLIENT: FileSender = FileSender {
        // SUNAT has no beta for guias de remision: they go to production
        urls: Urls {
            despatch: Urls::production().despatch,
            ..Urls::beta()
        },
        credentials: Credentials {
            username: "12345678959MODDATOS".to_string(),
            password: "MODDATOS".to_string(),
            ..Default::default()
        },
    };
}

pub fn defaults_base() -> Defaults {
    Defaults::new(NaiveDate::from_ymd_opt(2019, 12, 24).unwrap())
}

#[allow(dead_code)]
pub fn invoice_base() -> Invoice {
    Invoice {
        serie_numero: "F001-1".into(),
        proveedor: proveedor_base(),
        cliente: cliente_base(),
        ..Default::default()
    }
}

#[allow(dead_code)]
pub fn credit_note_base() -> CreditNote {
    CreditNote {
        serie_numero: "FC01-1".into(),
        comprobante_afectado_serie_numero: "F001-1".into(),
        descripcion_sustento: "mi sustento".into(),
        proveedor: proveedor_base(),
        cliente: cliente_base(),
        ..Default::default()
    }
}

#[allow(dead_code)]
pub fn debit_note_base() -> DebitNote {
    DebitNote {
        serie_numero: "FD01-1".into(),
        comprobante_afectado_serie_numero: "F001-1".into(),
        descripcion_sustento: "mi sustento".into(),
        proveedor: proveedor_base(),
        cliente: cliente_base(),
        ..Default::default()
    }
}

pub fn proveedor_base() -> Proveedor {
    Proveedor {
        ruc: "12345678912".into(),
        razon_social: "Softgreen S.A.C.".into(),
        ..Default::default()
    }
}

pub fn cliente_base() -> Cliente {
    Cliente {
        tipo_documento_identidad: Catalog6::Ruc.code().into(),
        numero_documento_identidad: "12121212121".into(),
        nombre: "Carlos Feria".into(),
        ..Default::default()
    }
}

fn sign_xml(xml: &str) -> Vec<u8> {
    let private_key_from_file = fs::read_to_string("tests/resources/certificates/private.key")
        .expect("Could not read private.key");
    let certificate_from_file = fs::read_to_string("tests/resources/certificates/public.cer")
        .expect("Could not read public.cer");

    let rsa_key_pair = RsaKeyPair::from_pem(&private_key_from_file, &certificate_from_file)
        .expect("Could not initialize RsaKeyPair");

    let signer = XSigner {
        xml_document: xml.to_string(),
    };
    let signed = signer.sign(&rsa_key_pair).expect("Could not sign document");
    assert_text_preserved(xml, &signed);
    signed
}

/// Signing must not alter the document's text (e.g. re-decode UTF-8 as Latin-1).
fn assert_text_preserved(xml: &str, signed: &[u8]) {
    let signed = std::str::from_utf8(signed).expect("Signed XML must be UTF-8");
    for word in xml
        .split(|c: char| c.is_ascii())
        .filter(|word| !word.is_empty())
    {
        assert!(
            signed.contains(word),
            "Signed XML lost the non-ASCII text {word:?}"
        );
    }
}

#[allow(dead_code)]
pub async fn assert_invoice(invoice: &mut Invoice, snapshot_filename: &str) {
    let xml_signed = render_and_check_invoice(invoice, snapshot_filename);
    assert_sunat(&xml_signed).await;
}

/// Same as [`assert_invoice`] but never sends the document to SUNAT.
/// Use it for large parameterized batches that SUNAT beta would rate-limit;
/// the snapshot and XSD assertions still run.
#[allow(dead_code)]
pub fn assert_invoice_offline(invoice: &mut Invoice, snapshot_filename: &str) {
    render_and_check_invoice(invoice, snapshot_filename);
}

fn render_and_check_invoice(invoice: &mut Invoice, snapshot_filename: &str) -> Vec<u8> {
    let defaults = defaults_base();
    dump_input(&defaults, &*invoice, snapshot_filename);
    invoice.enrich(&defaults).expect("Could not enrich");

    let xml = invoice.render().expect("Could not render invoice");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, INVOICE_XSD);
    xml_signed
}

#[allow(dead_code)]
pub async fn assert_credit_note(credit_note: &mut CreditNote, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*credit_note, snapshot_filename);
    credit_note.enrich(&defaults).expect("Could not enrich");

    let xml = credit_note.render().expect("Could not render credit note");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, CREDIT_NOTE_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_debit_note(debit_note: &mut DebitNote, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*debit_note, snapshot_filename);
    debit_note.enrich(&defaults).expect("Could not enrich");

    let xml = debit_note.render().expect("Could not render debit note");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, DEBIT_NOTE_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
/// Also used for [`Reversion`], which shares the VoidedDocuments template and XSD
pub async fn assert_voided_documents(
    doc: &mut (impl Enrich + Renderer + serde::Serialize),
    snapshot_filename: &str,
) {
    let defaults = defaults_base();
    dump_input(&defaults, &*doc, snapshot_filename);
    doc.enrich(&defaults).expect("Could not enrich");

    let xml = doc.render().expect("Could not render voided documents");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, VOIDED_DOCUMENTS_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_summary_documents(doc: &mut SummaryDocuments, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*doc, snapshot_filename);
    doc.enrich(&defaults).expect("Could not enrich");

    let xml = doc.render().expect("Could not render summary documents");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, SUMMARY_DOCUMENTS_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_perception(doc: &mut Perception, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*doc, snapshot_filename);
    doc.enrich(&defaults).expect("Could not enrich");

    let xml = doc.render().expect("Could not render perception");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, PERCEPTION_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_retention(doc: &mut Retention, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*doc, snapshot_filename);
    doc.enrich(&defaults).expect("Could not enrich");

    let xml = doc.render().expect("Could not render retention");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, RETENTION_XSD);
    assert_sunat(&xml_signed).await;
}

#[allow(dead_code)]
pub async fn assert_despatch_advice(doc: &mut DespatchAdvice, snapshot_filename: &str) {
    let defaults = defaults_base();
    dump_input(&defaults, &*doc, snapshot_filename);
    doc.enrich(&defaults).expect("Could not enrich");

    let xml = doc.render().expect("Could not render despatch advice");

    assert_snapshot(&xml, snapshot_filename);

    let xml_signed = sign_xml(&xml);
    assert_xsd(&xml_signed, DESPATCH_ADVICE_XSD);
    // SUNAT send skipped: DespatchAdvice uses REST endpoint, not SOAP
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| matches!(value.to_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(false)
}

/// `XBUILDER_DUMP_INPUT=<dir>` writes the input of each snapshot as the
/// `{"document": {"kind", "spec"}, "defaults"}` JSON of the bindings' `build`
/// op, next to its snapshot path (relative to `tests/resources/e2e/`). It feeds
/// the conformance corpus of the bindings.
fn dump_input<T: serde::Serialize>(defaults: &Defaults, document: &T, snapshot_filename: &str) {
    let Ok(dir) = std::env::var("XBUILDER_DUMP_INPUT") else {
        return;
    };
    // The test documents are the model structs themselves, e.g. `Invoice`
    let kind = std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .expect("type name");
    let input = serde_json::json!({
        "document": { "kind": kind, "spec": document },
        "defaults": {
            "fecha_emision": defaults.fecha_emision,
            "icb_tasa": defaults.icb_tasa,
            "igv_tasa": defaults.igv_tasa,
            "ivap_tasa": defaults.ivap_tasa,
        },
    });
    let relative = snapshot_filename
        .strip_prefix("tests/resources/e2e/")
        .unwrap_or(snapshot_filename);
    let path = std::path::Path::new(&dir)
        .join(relative)
        .with_extension("json");
    fs::create_dir_all(path.parent().expect("parent dir")).expect("Could not create dump dir");
    let json = serde_json::to_string_pretty(&input).expect("Could not serialize input");
    fs::write(&path, json).expect("Could not write input dump");
}

fn assert_snapshot(expected: &str, snapshot_filename: &str) {
    // XBUILDER_SNAPSHOT_UPDATE=true overwrites snapshots with the rendered XML
    if env_flag("XBUILDER_SNAPSHOT_UPDATE") {
        fs::write(snapshot_filename, expected)
            .unwrap_or_else(|err| panic!("Could not write snapshot {snapshot_filename}: {err}"));
        return;
    }

    let snapshot_file_content = fs::read_to_string(snapshot_filename)
        .unwrap_or_else(|err| panic!("Could not read snapshot {snapshot_filename}: {err}"));

    assert_eq!(
        expected, snapshot_file_content,
        "File {} does not match",
        snapshot_filename
    );
}

fn assert_xsd(xml: &Vec<u8>, schema: &str) {
    let mut xsdparser = SchemaParserContext::from_file(schema);
    let xsd = SchemaValidationContext::from_parser(&mut xsdparser);

    if let Err(errors) = xsd {
        for err in &errors {
            println!("{}", err.message.as_ref().unwrap());
        }

        panic!("Failed to parse schema");
    }

    let mut xsd = xsd.unwrap();

    let xml_document = Parser::default().parse_string(xml).unwrap();
    if let Err(errors) = xsd.validate_document(&xml_document) {
        for err in &errors {
            println!("{}", err.message.as_ref().unwrap());
        }

        panic!("Invalid XML according to XSD schema");
    }
}

async fn assert_sunat(xml: &[u8]) {
    // XBUILDER_SKIP_SUNAT=true skips sending to SUNAT beta (snapshot + XSD still run)
    if env_flag("XBUILDER_SKIP_SUNAT") {
        return;
    }

    let xml_file = UblFile::from_bytes(xml).expect("Signed XML must be valid");

    let result = CLIENT
        .send_file(&xml_file)
        .await
        .expect("Could not get a valid response");

    let document_type = xml_file
        .metadata()
        .expect("Could not extract xml metadata")
        .document_type;

    match document_type.as_str() {
        DocumentType::VOIDED_DOCUMENTS | DocumentType::SUMMARY_DOCUMENTS => {
            let result = match &result.response {
                SendFileAggregatedResponse::Cdr(_, _) => false,
                SendFileAggregatedResponse::Ticket(_) => true,
                SendFileAggregatedResponse::Fault(fault) => {
                    println!("SUNAT fault: code={} message={}", fault.code, fault.message);
                    false
                }
            };
            assert!(result, "Expected Ticket response from SUNAT")
        }
        _ => {
            let result = match result.response {
                SendFileAggregatedResponse::Cdr(_, cdr_metadata) => {
                    assert_eq!("0", cdr_metadata.response_code);
                    assert_eq!(Vec::<String>::new(), cdr_metadata.notes);
                    true
                }
                SendFileAggregatedResponse::Ticket(_) => false,
                SendFileAggregatedResponse::Fault(_) => false,
            };
            assert!(result)
        }
    };
}
