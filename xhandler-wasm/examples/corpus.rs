//! Writes the conformance corpus that every binding (Go, Python, Java) runs
//! through its own `xh_invoke` glue.
//!
//! Each case is `<out>/<op>/<name>.json` with `{"input": ..., "expected": ...}`,
//! where `expected` is the whole response: `{"ok": ...}` or
//! `{"error": {"kind": ...}}` (error messages are not compared).
//!
//! Usage: corpus <xbuilder input dump dir> <workspace root> <out dir>
//!
//! The dump comes from `XBUILDER_DUMP_INPUT=<dir> cargo test -p xbuilder`: the
//! `build` input of every snapshot test. The snapshots are the source of
//! truth: the run fails if the native `invoke` does not reproduce one of them.
//! The `enrich` cases record what the native `invoke` returns, so they check
//! that the bindings carry every field, not the enrichment itself (the
//! xbuilder tests do). The `sign` cases are the byte-exact signatures of the
//! xsigner tests: RSA PKCS#1 v1.5 is deterministic. The sender cases
//! (prepare_*, request, parse) record what the native `invoke` returns for the
//! xsender test files, so the bindings exchange the same requests and answers
//! (the xsender tests check their content).

use std::fs;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, dump_dir, root, out_dir] = args.as_slice() else {
        eprintln!("usage: corpus <dump dir> <workspace root> <out dir>");
        std::process::exit(2);
    };
    let (dump_dir, root, out_dir) = (Path::new(dump_dir), Path::new(root), Path::new(out_dir));
    let snapshots_dir = root.join("xbuilder/tests/resources/e2e");

    let mut corpus = Corpus::new(out_dir);

    for dump in json_files(dump_dir) {
        let relative = dump.strip_prefix(dump_dir).expect("inside dump dir");
        let input = read_json(&dump);
        let snapshot = fs::read_to_string(snapshots_dir.join(relative).with_extension("xml"))
            .unwrap_or_else(|e| panic!("snapshot of {}: {e}", relative.display()));

        corpus.check("build", relative, input.clone(), json!({ "ok": snapshot }));
        let enriched = invoke("enrich", &input);
        corpus.check("enrich", relative, input, enriched);
    }

    sign_cases(&mut corpus, &root.join("xsigner/resources/test"));
    sender_cases(&mut corpus, &root.join("xsender/resources/test"));

    for (op, name, input, kind) in error_cases() {
        corpus.check(
            op,
            Path::new(name),
            input,
            json!({ "error": { "kind": kind } }),
        );
    }

    corpus.finish();
}

fn sign_cases(corpus: &mut Corpus, resources: &Path) {
    let read = |name: &str| fs::read(resources.join(name)).expect("xsigner resource");
    let text = |name: &str| String::from_utf8(read(name)).expect("UTF-8 resource");
    let pem = json!({ "pem": {
        "private_key": text("private.key"),
        "certificate": text("public.cer"),
    }});
    let pkcs12 = |name: &str, password: &str| json!({ "pkcs12": { "der": STANDARD.encode(read(name)), "password": password } });
    let sign = |xml: &[u8], key: &Value| json!({ "xml": STANDARD.encode(xml), "key": key });
    let ok = |signed: &[u8]| json!({ "ok": STANDARD.encode(signed) });

    // signed/<name>.xml -> signed/<name>.signed.xml, signed with private.key
    let mut signed = files_with_suffix(&resources.join("signed"), ".signed.xml");
    signed.sort();
    for expected in signed {
        let file = expected.file_name().unwrap().to_str().unwrap();
        let name = file.strip_suffix(".signed.xml").unwrap();
        let source = read(&format!("signed/{name}.xml"));
        let expected = fs::read(&expected).unwrap();
        corpus.check(
            "sign",
            Path::new(&format!("pem/{name}")),
            sign(&source, &pem),
            ok(&expected),
        );
    }

    // The 3DES keystore holds the same key as private.key
    let invoice = read("signed/invoice_no_template.xml");
    corpus.check(
        "sign",
        Path::new("pkcs12/legacy_3des"),
        sign(&invoice, &pkcs12("legacy_3des.pfx", "password")),
        ok(&read("signed/invoice_no_template.signed.xml")),
    );
    // Another key: the expected output records the native one
    let llama = pkcs12("LLAMA-PE-CERTIFICADO-DEMO-10467793549.pfx", "password");
    let expected = invoke("sign", &sign(&invoice, &llama));
    corpus.check(
        "sign",
        Path::new("pkcs12/llama"),
        sign(&invoice, &llama),
        expected,
    );

    // ISO-8859-1 bytes sign like their UTF-8 twin: the output is UTF-8
    let utf8 = text("signed/special_characters.xml");
    let latin1: Vec<u8> = utf8
        .replacen("encoding=\"UTF-8\"", "encoding=\"ISO-8859-1\"", 1)
        .chars()
        .map(|c| u8::try_from(u32::from(c)).expect("Latin-1 text"))
        .collect();
    corpus.check(
        "sign",
        Path::new("pem/special_characters_iso_8859_1"),
        sign(&latin1, &pem),
        ok(&read("signed/special_characters.signed.xml")),
    );

    let error = |kind: &str| json!({ "error": { "kind": kind } });
    corpus.check(
        "sign",
        Path::new("errors/not_utf8_nor_declared_latin1"),
        sign(b"<Invoice>Pe\xf1a</Invoice>", &pem),
        error("invalid_input"),
    );
    corpus.check(
        "sign",
        Path::new("errors/garbage_pem"),
        sign(
            &invoice,
            &json!({ "pem": { "private_key": "not a key", "certificate": text("public.cer") } }),
        ),
        error("invalid_key"),
    );
    let mut truncated = read("legacy_3des.pfx");
    truncated.truncate(100);
    corpus.check(
        "sign",
        Path::new("errors/truncated_pkcs12"),
        sign(
            &invoice,
            &json!({ "pkcs12": { "der": STANDARD.encode(truncated), "password": "password" } }),
        ),
        error("invalid_key"),
    );
    corpus.check(
        "sign",
        Path::new("errors/wrong_password"),
        sign(&invoice, &pkcs12("legacy_3des.pfx", "wrong")),
        error("invalid_key"),
    );
    let other_key = json!({ "pem": {
        "private_key": text("other_private.key"),
        "certificate": text("public.cer"),
    }});
    corpus.check(
        "sign",
        Path::new("errors/certificate_of_another_key"),
        sign(&invoice, &other_key),
        error("invalid_key"),
    );
    let in_use = br#"<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2" xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2"><ext:UBLExtensions><ext:UBLExtension><ext:ExtensionContent><x/></ext:ExtensionContent></ext:UBLExtension></ext:UBLExtensions></Invoice>"#;
    corpus.check(
        "sign",
        Path::new("errors/extension_content_in_use"),
        sign(in_use, &pem),
        error("invalid_input"),
    );
    corpus.check(
        "sign",
        Path::new("errors/xml_not_base64"),
        json!({ "xml": "<Invoice/>", "key": pem }),
        error("invalid_input"),
    );
}

fn sender_cases(corpus: &mut Corpus, resources: &Path) {
    let read = |name: &str| fs::read(resources.join(name)).expect("xsender resource");
    let text = |name: &str| String::from_utf8(read(name)).expect("UTF-8 resource");
    let xml = |name: &str| STANDARD.encode(read(name));
    let production = json!({
        "invoice": "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService",
        "perception_retention": "https://e-factura.sunat.gob.pe/ol-ti-itemision-otroscpe-gem/billService",
        "despatch": "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem",
    });
    let beta = json!({
        "invoice": "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService",
        "perception_retention": "https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService",
        "despatch": "",
    });
    let mut native = |op: &str, name: &str, input: Value| {
        let expected = invoke(op, &input);
        corpus.check(op, Path::new(name), input, without_message(expected));
    };

    for file in [
        "F001-1.xml",
        "150101-F001-11.xml",
        "RA-20200328-1.xml",
        "P001-1.xml",
        "R001-1.xml",
        "despatch-advice-T.xml",
        "despatch-advice-V.xml",
        "invoice_custom_prefixes.xml",
    ] {
        let name = file.trim_end_matches(".xml");
        native(
            "prepare_send_file",
            name,
            json!({ "xml": xml(file), "urls": production }),
        );
    }
    native(
        "prepare_send_file",
        "errors/despatch_without_url",
        json!({ "xml": xml("despatch-advice-T.xml"), "urls": beta }),
    );
    native(
        "prepare_send_file",
        "errors/not_a_document",
        json!({ "xml": STANDARD.encode("<html/>"), "urls": beta }),
    );
    native(
        "prepare_send_pack",
        "invoices",
        json!({ "name": "12345678912-20200328-1", "files": [xml("F001-1.xml"), xml("150101-F001-11.xml")], "urls": beta }),
    );
    native(
        "verify_ticket_target",
        "voided_documents",
        json!({ "xml": xml("RA-20200328-1.xml"), "urls": beta }),
    );
    native(
        "verify_ticket_target",
        "despatch_advice",
        json!({ "xml": xml("despatch-advice-T.xml"), "urls": production }),
    );
    native(
        "verify_ticket_target",
        "errors/invoice",
        json!({ "xml": xml("F001-1.xml"), "urls": beta }),
    );

    let soap_bill = json!({ "protocol": "soap", "url": beta["invoice"], "action": "bill" });
    let soap = json!({ "protocol": "soap", "url": beta["invoice"] });
    let rest = json!({ "protocol": "rest", "url": production["despatch"] });
    let file = json!({ "filename": "12345678912-01-F001-1.zip", "zip": "UEsFBgAAAAAAAAAAAAAAAAAAAAAAAA==" });
    let document =
        json!({ "ruc": "12345678912", "tipo_comprobante": "01", "serie": "F001", "numero": 1 });
    let calls = [
        (
            "send_file_soap",
            json!({ "send_file": { "target": soap_bill, "file": file } }),
        ),
        (
            "send_file_rest",
            json!({ "send_file": { "target": rest, "file": file } }),
        ),
        (
            "verify_ticket_soap",
            json!({ "verify_ticket": { "target": soap, "ticket": "1703154974517" } }),
        ),
        (
            "verify_ticket_rest",
            json!({ "verify_ticket": { "target": rest, "ticket": "a7b3c1d2-0001" } }),
        ),
        (
            "get_status",
            json!({ "get_status": { "url": "https://sunat/billConsultService", "document": document } }),
        ),
        (
            "get_status_cdr",
            json!({ "get_status_cdr": { "url": "https://sunat/billConsultService", "document": document } }),
        ),
        (
            "validate_cdp",
            json!({ "validate_cdp": { "url": "https://sunat/billValidService", "cdp": {
                "ruc": "12345678912", "tipo_comprobante": "01", "serie": "F001", "numero": 1,
                "tipo_documento_identidad_receptor": "6", "numero_documento_identidad_receptor": "12121212121",
                "fecha_emision": "24/12/2019", "importe_total": "118.00", "numero_autorizacion": null,
            }}}),
        ),
        (
            "validate_file",
            json!({ "validate_file": { "url": "https://sunat/billValidService", "file": file } }),
        ),
        ("token", json!("token")),
    ];
    let credentials = json!({
        "username": "12345678959MODDATOS",
        "password": "MODDATOS",
        "client_id": "client-id",
        "client_secret": "client-secret",
    });
    for (name, call) in &calls {
        native(
            "request",
            name,
            json!({ "call": call, "credentials": credentials, "token": "the-token" }),
        );
    }
    native(
        "request",
        "errors/rest_without_token",
        json!({ "call": calls[1].1, "credentials": credentials }),
    );

    let call = |name: &str| calls.iter().find(|(n, _)| *n == name).unwrap().1.clone();
    let parse = |call_name: &str, status: u16, body: String| json!({ "call": call(call_name), "status": status, "body": body });
    let answers = [
        ("send_file_cdr", parse("send_file_soap", 200, text("bill_service_response_ok.xml"))),
        ("send_file_ticket", parse("send_file_soap", 200, text("bill_service_response_ticket.xml"))),
        ("send_file_fault", parse("send_file_soap", 500, text("bill_service_response_fault.xml"))),
        ("send_file_rejected", parse("send_file_soap", 500, text("bill_service_response_fault_rechazo.xml"))),
        ("verify_ticket_cdr", parse("verify_ticket_soap", 200, text("get_status_response_ok.xml"))),
        ("verify_ticket_in_progress", parse("verify_ticket_soap", 200, text("get_status_response_in_progress.xml"))),
        ("verify_ticket_fault", parse("verify_ticket_soap", 500, text("get_status_response_fault.xml"))),
        ("get_status_cdr", parse("get_status_cdr", 200, text("get_status_cdr_response.xml"))),
        ("validate_cdp", parse("validate_cdp", 200, text("valida_cdp_response.xml"))),
        ("send_file_rest_ticket", parse("send_file_rest", 200, r#"{"numTicket":"a7b3c1d2-0001"}"#.into())),
        ("send_file_rest_error", parse("send_file_rest", 422, r#"{"cod":"422","msg":"Unprocessable Entity","errors":[{"cod":"1033","msg":"El comprobante fue registrado previamente"}]}"#.into())),
        ("verify_ticket_rest_in_progress", parse("verify_ticket_rest", 200, r#"{"codRespuesta":"98"}"#.into())),
        ("verify_ticket_rest_error", parse("verify_ticket_rest", 200, r#"{"codRespuesta":"99","error":{"numError":"2223","desError":"El documento ya fue informado"}}"#.into())),
        ("token", parse("token", 200, r#"{"access_token":"abc","expires_in":600}"#.into())),
        ("errors/html_page", parse("send_file_soap", 503, "<html><body>Service Unavailable</body></html>".into())),
        ("errors/token_rejected", parse("token", 401, r#"{"error":"invalid_client"}"#.into())),
    ];
    for (name, input) in answers {
        native("parse", name, input);
    }
}

/// Error messages are not compared: they may change without breaking anyone.
fn without_message(mut response: Value) -> Value {
    if let Some(error) = response.get_mut("error") {
        error
            .as_object_mut()
            .expect("error object")
            .remove("message");
    }
    response
}

fn error_cases() -> Vec<(&'static str, &'static str, Value, &'static str)> {
    let invoice = |igv_tipo: &str| {
        json!({
            "document": {
                "kind": "Invoice",
                "spec": {
                    "serie_numero": "F001-1",
                    "proveedor": { "ruc": "12345678912", "razon_social": "Softgreen S.A.C." },
                    "cliente": {
                        "tipo_documento_identidad": "6",
                        "numero_documento_identidad": "12121212121",
                        "nombre": "Carlos Feria"
                    },
                    "detalles": [{ "descripcion": "Item", "cantidad": "1", "precio": "100", "igv_tipo": igv_tipo }]
                }
            },
            "defaults": { "fecha_emision": "2019-12-24" }
        })
    };
    vec![
        (
            "build",
            "errors/invalid_catalog_code",
            invoice("XX"),
            "validation",
        ),
        (
            "build",
            "errors/unknown_kind",
            json!({ "document": { "kind": "Nope", "spec": {} }, "defaults": { "fecha_emision": "2019-12-24" } }),
            "invalid_input",
        ),
        (
            "build",
            "errors/invalid_date",
            json!({ "document": invoice("10")["document"], "defaults": { "fecha_emision": "24/12/2019" } }),
            "invalid_input",
        ),
        ("nope", "errors/unknown_op", json!({}), "invalid_request"),
    ]
}

struct Corpus<'a> {
    out_dir: &'a Path,
    cases: usize,
    failures: Vec<String>,
}

impl<'a> Corpus<'a> {
    fn new(out_dir: &'a Path) -> Self {
        Self {
            out_dir,
            cases: 0,
            failures: vec![],
        }
    }

    /// Writes the case after checking that the native `invoke` agrees with it.
    fn check(&mut self, op: &str, name: &Path, input: Value, expected: Value) {
        let mut actual = invoke(op, &input);
        if let Some(error) = actual.get_mut("error") {
            error
                .as_object_mut()
                .expect("error object")
                .remove("message");
        }
        if actual != expected {
            self.failures
                .push(format!("{op}/{}: got {actual}", name.display()));
        }

        let path = self.out_dir.join(op).join(name).with_extension("json");
        fs::create_dir_all(path.parent().expect("parent")).expect("create dir");
        let case = json!({ "input": input, "expected": expected });
        fs::write(&path, serde_json::to_string_pretty(&case).expect("json")).expect("write case");
        self.cases += 1;
    }

    fn finish(self) {
        println!("{} cases written to {}", self.cases, self.out_dir.display());
        if !self.failures.is_empty() {
            eprintln!(
                "{} cases do not match:\n{}",
                self.failures.len(),
                self.failures.join("\n")
            );
            std::process::exit(1);
        }
    }
}

fn invoke(op: &str, input: &Value) -> Value {
    let request = serde_json::to_vec(&json!({ "op": op, "input": input })).expect("json");
    serde_json::from_slice(&xhandler_wasm::invoke(&request)).expect("response json")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read")).expect("json")
}

fn files_with_suffix(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.to_str().is_some_and(|p| p.ends_with(suffix)))
        .collect()
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            files.extend(json_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "json") {
            files.push(path);
        }
    }
    files.sort();
    files
}
