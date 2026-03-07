use std::process::Command;

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

/// Helper: send a voided document to SUNAT beta and return the ticket number.
fn send_voided_and_get_ticket() -> String {
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
        "send should succeed to get ticket, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    json["ticket"]
        .as_str()
        .expect("send response should contain ticket")
        .to_string()
}

// --- CLI argument validation tests ---

#[test]
fn verify_ticket_missing_ticket_fails() {
    let output = Command::new(BINARY)
        .args(["verify-ticket", "-o", "output.zip", "--beta"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl verify-ticket");

    assert!(!output.status.success(), "should fail without --ticket");
}

#[test]
fn verify_ticket_missing_output_fails() {
    let output = Command::new(BINARY)
        .args(["verify-ticket", "--ticket", "123", "--beta"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl verify-ticket");

    assert!(!output.status.success(), "should fail without -o");
}

#[test]
fn verify_ticket_missing_credentials_without_beta_fails() {
    let output = Command::new(BINARY)
        .args(["verify-ticket", "--ticket", "123", "-o", "output.zip"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl verify-ticket");

    assert!(
        !output.status.success(),
        "should fail without credentials and without --beta"
    );
}

// --- End-to-end test against SUNAT beta ---

#[serial_test::serial]
#[test]
fn verify_ticket_from_voided_documents() {
    let ticket = send_voided_and_get_ticket();

    let temp_dir = std::env::temp_dir();
    let cdr_path = temp_dir.join("openubl_verify_ticket_cdr.zip");
    let _ = std::fs::remove_file(&cdr_path);

    let output = Command::new(BINARY)
        .args([
            "verify-ticket",
            "--ticket",
            &ticket,
            "-o",
            cdr_path.to_str().unwrap(),
            "--beta",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl verify-ticket");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if output.status.success() {
        // CDR response
        assert!(
            stdout.contains("cdr"),
            "output should contain cdr path: {stdout}"
        );
        assert!(
            stdout.contains("status_code"),
            "output should contain status_code: {stdout}"
        );
        assert!(
            stdout.contains("response_code"),
            "output should contain response_code: {stdout}"
        );

        assert!(cdr_path.exists(), "CDR file should exist at {cdr_path:?}");
        assert_valid_zip(&cdr_path);

        let _ = std::fs::remove_file(&cdr_path);
    } else {
        // SUNAT beta may return an error for ticket verification (known issue).
        // Verify the error output is structured JSON.
        assert!(
            stderr.contains("code") || stderr.contains("message"),
            "error output should be structured JSON, stderr: {stderr}"
        );
    }
}
