use std::process::Command;

use openubl_cli::commands::send::SendArgs;

const BINARY: &str = env!("CARGO_BIN_EXE_openubl");
const E2E_RESOURCES: &str = "../xsender/tests/resources/e2e";

fn assert_valid_zip(path: &std::path::Path) {
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.len() > 4, "zip file too small: {} bytes", bytes.len());
    assert_eq!(
        &bytes[..4],
        b"PK\x03\x04",
        "file does not start with zip magic bytes"
    );
}

// --- End-to-end send tests against SUNAT beta ---

#[serial_test::serial]
#[test]
fn send_invoice_to_sunat_beta() {
    // Copy input XML to temp so the default CDR .zip goes to temp dir
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join("openubl_send_test_invoice.xml");
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::copy(
        manifest.join(format!("{E2E_RESOURCES}/12345678912-01-F001-1.xml")),
        &input_path,
    )
    .unwrap();

    let default_cdr_path = temp_dir.join("openubl_send_test_invoice.xml.zip");

    let output = Command::new(BINARY)
        .args(["send", "-f", input_path.to_str().unwrap(), "--beta"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(
        output.status.success(),
        "send should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("cdr"),
        "output should contain cdr path: {stdout}"
    );
    assert!(
        stdout.contains("response_code"),
        "output should contain response_code: {stdout}"
    );
    assert!(
        stdout.contains("description"),
        "output should contain description: {stdout}"
    );

    // CDR should be saved to default path (input_file + .zip)
    assert!(
        default_cdr_path.exists(),
        "default CDR file should exist at {default_cdr_path:?}"
    );
    assert_valid_zip(&default_cdr_path);

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&default_cdr_path);
}

#[serial_test::serial]
#[test]
fn send_invoice_to_sunat_beta_saves_cdr() {
    let temp_dir = std::env::temp_dir();
    let cdr_path = temp_dir.join("openubl_send_test_cdr.zip");

    let output = Command::new(BINARY)
        .args([
            "send",
            "-f",
            &format!("{E2E_RESOURCES}/12345678912-01-F001-1.xml"),
            "-o",
            cdr_path.to_str().unwrap(),
            "--beta",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(
        output.status.success(),
        "send should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // CDR file should have been saved as a valid zip
    assert!(cdr_path.exists(), "CDR file should exist at {cdr_path:?}");
    assert_valid_zip(&cdr_path);

    let _ = std::fs::remove_file(&cdr_path);
}

#[serial_test::serial]
#[test]
fn send_voided_documents_to_sunat_beta() {
    let output = Command::new(BINARY)
        .args([
            "send",
            "-f",
            &format!("{E2E_RESOURCES}/12345678912-RA-20200328-1.xml"),
            "--beta",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(
        output.status.success(),
        "send should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    // VoidedDocuments returns a ticket
    assert!(
        stdout.contains("ticket"),
        "output should contain ticket: {stdout}"
    );
}

// --- CLI argument validation tests ---

#[test]
fn send_missing_file_arg_fails() {
    let output = Command::new(BINARY)
        .args(["send", "--username", "user", "--password", "pass"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(!output.status.success(), "should fail without -f");
}

#[test]
fn send_missing_credentials_without_beta_fails() {
    let output = Command::new(BINARY)
        .args(["send", "-f", "some.xml"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(
        !output.status.success(),
        "should fail without credentials and without --beta"
    );
}

#[test]
fn send_nonexistent_file_fails() {
    let output = Command::new(BINARY)
        .args(["send", "-f", "nonexistent.xml", "--beta"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl send");

    assert!(!output.status.success(), "should fail for nonexistent file");
}

// --- URL and credential resolution unit tests ---

fn make_send_args(beta: bool) -> SendArgs {
    SendArgs {
        input_file: String::new(),
        output_file: None,
        username: None,
        password: None,
        url_invoice: None,
        url_perception_retention: None,
        url_despatch: None,
        beta,
        no_interactive: false,
    }
}

#[test]
fn resolve_urls_prod() {
    let args = make_send_args(false);
    let urls = args.resolve_urls();

    assert!(
        urls.invoice.contains("e-factura.sunat.gob.pe"),
        "prod invoice URL: {}",
        urls.invoice
    );
    assert!(
        urls.perception_retention.contains("e-factura.sunat.gob.pe"),
        "prod perception_retention URL: {}",
        urls.perception_retention
    );
    assert!(
        urls.despatch.contains("api-cpe.sunat.gob.pe"),
        "prod despatch URL: {}",
        urls.despatch
    );
}

#[test]
fn resolve_urls_beta() {
    let args = make_send_args(true);
    let urls = args.resolve_urls();

    assert!(
        urls.invoice.contains("e-beta.sunat.gob.pe"),
        "beta invoice URL: {}",
        urls.invoice
    );
    assert!(
        urls.perception_retention.contains("e-beta.sunat.gob.pe"),
        "beta perception_retention URL: {}",
        urls.perception_retention
    );
    assert!(
        urls.despatch.contains("api-cpe.sunat.gob.pe"),
        "beta despatch URL: {}",
        urls.despatch
    );
}

#[test]
fn resolve_urls_custom_override() {
    let args = SendArgs {
        url_invoice: Some("https://custom-invoice.example.com".to_string()),
        url_perception_retention: Some("https://custom-pr.example.com".to_string()),
        url_despatch: Some("https://custom-despatch.example.com".to_string()),
        ..make_send_args(false)
    };
    let urls = args.resolve_urls();

    assert_eq!(urls.invoice, "https://custom-invoice.example.com");
    assert_eq!(urls.perception_retention, "https://custom-pr.example.com");
    assert_eq!(urls.despatch, "https://custom-despatch.example.com");
}

#[test]
fn resolve_urls_custom_overrides_beta() {
    let args = SendArgs {
        url_invoice: Some("https://custom.example.com".to_string()),
        ..make_send_args(true)
    };
    let urls = args.resolve_urls();

    assert_eq!(urls.invoice, "https://custom.example.com");
    assert!(
        urls.perception_retention.contains("e-beta.sunat.gob.pe"),
        "should fall back to beta URL: {}",
        urls.perception_retention
    );
}

#[test]
fn resolve_credentials_beta_defaults() {
    let args = make_send_args(true);
    let creds = args.resolve_credentials().unwrap();

    assert_eq!(creds.username, "12345678959MODDATOS");
    assert_eq!(creds.password, "MODDATOS");
}

#[test]
fn resolve_credentials_beta_with_override() {
    let args = SendArgs {
        username: Some("custom_user".to_string()),
        password: Some("custom_pass".to_string()),
        ..make_send_args(true)
    };
    let creds = args.resolve_credentials().unwrap();

    assert_eq!(creds.username, "custom_user");
    assert_eq!(creds.password, "custom_pass");
}

#[test]
fn resolve_credentials_prod_missing_fails() {
    let args = make_send_args(false);
    assert!(args.resolve_credentials().is_err());
}

#[test]
fn resolve_credentials_prod_with_values() {
    let args = SendArgs {
        username: Some("my_user".to_string()),
        password: Some("my_pass".to_string()),
        ..make_send_args(false)
    };
    let creds = args.resolve_credentials().unwrap();

    assert_eq!(creds.username, "my_user");
    assert_eq!(creds.password, "my_pass");
}
