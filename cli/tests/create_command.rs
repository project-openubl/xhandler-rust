use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_openubl");
const RESOURCES: &str = "tests/resources";

#[test]
fn create_invoice_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/invoice.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("<Invoice"),
        "output should contain <Invoice"
    );
    assert!(
        stdout.contains("F001-1"),
        "output should contain serie_numero"
    );
    assert!(stdout.contains("12345678912"), "output should contain RUC");
    assert!(
        stdout.contains("Item1"),
        "output should contain item description"
    );
}

#[test]
fn create_invoice_from_json() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/invoice.json")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("<Invoice"),
        "output should contain <Invoice"
    );
}

#[test]
fn create_invoice_to_file() {
    let temp_dir = std::env::temp_dir();
    let output_file = temp_dir.join("openubl_test_invoice.xml");

    let output = Command::new(BINARY)
        .args([
            "create",
            "-f",
            &format!("{RESOURCES}/invoice.yaml"),
            "-o",
            output_file.to_str().unwrap(),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    // stdout should be empty when writing to file
    assert!(
        output.stdout.is_empty(),
        "stdout should be empty when -o is used"
    );

    // File should exist and contain XML
    let content = std::fs::read_to_string(&output_file).unwrap();
    assert!(content.contains("<Invoice"), "file should contain <Invoice");

    let _ = std::fs::remove_file(&output_file);
}

#[test]
fn create_invoice_dry_run() {
    let output = Command::new(BINARY)
        .args([
            "create",
            "-f",
            &format!("{RESOURCES}/invoice.yaml"),
            "--dry-run",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    // dry-run prints enriched JSON to stderr
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("serie_numero"),
        "dry-run should output enriched JSON"
    );
    assert!(
        stderr.contains("F001-1"),
        "dry-run should contain serie_numero value"
    );

    // stdout should be empty (no XML rendered)
    assert!(
        output.stdout.is_empty(),
        "stdout should be empty in dry-run mode"
    );
}

#[test]
fn create_credit_note_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/credit_note.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("<CreditNote"),
        "output should contain <CreditNote"
    );
}

#[test]
fn create_debit_note_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/debit_note.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("<DebitNote"),
        "output should contain <DebitNote"
    );
}

#[test]
fn create_despatch_advice_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/despatch_advice.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("<DespatchAdvice"),
        "output should contain <DespatchAdvice"
    );
}

#[test]
fn create_perception_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/perception.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Perception"),
        "output should contain Perception"
    );
}

#[test]
fn create_retention_from_yaml() {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/retention.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Retention"),
        "output should contain Retention"
    );
}

#[test]
fn create_summary_documents_from_yaml() {
    let output = Command::new(BINARY)
        .args([
            "create",
            "-f",
            &format!("{RESOURCES}/summary_documents.yaml"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("SummaryDocuments"),
        "output should contain SummaryDocuments"
    );
}

#[test]
fn create_voided_documents_from_yaml() {
    let output = Command::new(BINARY)
        .args([
            "create",
            "-f",
            &format!("{RESOURCES}/voided_documents.yaml"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("VoidedDocuments"),
        "output should contain VoidedDocuments"
    );
}

#[test]
fn create_nonexistent_file_fails() {
    let output = Command::new(BINARY)
        .args(["create", "-f", "nonexistent.yaml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(!output.status.success(), "should fail for nonexistent file");
}

#[test]
fn create_missing_file_arg_fails() {
    let output = Command::new(BINARY)
        .args(["create"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl");

    assert!(!output.status.success(), "should fail without -f");
}
