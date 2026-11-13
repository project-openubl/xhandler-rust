//! Writes the conformance corpus that every binding (Go, Python, Java) runs
//! through its own `xh_invoke` glue.
//!
//! Each case is `<out>/<op>/<name>.json` with `{"input": ..., "expected": ...}`,
//! where `expected` is the whole response: `{"ok": ...}` or
//! `{"error": {"kind": ...}}` (error messages are not compared).
//!
//! Usage: corpus <xbuilder input dump dir> <xbuilder snapshots dir> <out dir>
//!
//! The dump comes from `XBUILDER_DUMP_INPUT=<dir> cargo test -p xbuilder`. The
//! snapshots are the source of truth: the run fails if the native `invoke`
//! does not reproduce one of them.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

/// The date of `defaults_base()` in the xbuilder tests; the rates are the defaults.
const DEFAULTS: &str = r#"{"fecha_emision": "2019-12-24"}"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, dump_dir, snapshots_dir, out_dir] = args.as_slice() else {
        eprintln!("usage: corpus <dump dir> <snapshots dir> <out dir>");
        std::process::exit(2);
    };
    let (dump_dir, snapshots_dir, out_dir) = (
        Path::new(dump_dir),
        Path::new(snapshots_dir),
        Path::new(out_dir),
    );

    let mut corpus = Corpus::new(out_dir);
    let defaults: Value = serde_json::from_str(DEFAULTS).expect("defaults");

    for dump in json_files(dump_dir) {
        let relative = dump.strip_prefix(dump_dir).expect("inside dump dir");
        let document: Value = read_json(&dump);
        let snapshot = fs::read_to_string(snapshots_dir.join(relative).with_extension("xml"))
            .unwrap_or_else(|e| panic!("snapshot of {}: {e}", relative.display()));
        let input = json!({ "document": document, "defaults": defaults });

        corpus.check("build", relative, input.clone(), json!({ "ok": snapshot }));
        let enriched = invoke("enrich", &input);
        corpus.check("enrich", relative, input, enriched);
    }

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
            "errors/missing_defaults",
            json!({ "document": invoice("10")["document"] }),
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
