use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_openubl");
const RESOURCES: &str = "tests/resources";

fn assert_valid_xml(path: &std::path::Path) {
    let content = std::fs::read_to_string(path).unwrap();
    assert!(
        content.starts_with("<?xml") || content.starts_with('<'),
        "file should contain XML: {path:?}"
    );
}

fn assert_valid_zip(path: &std::path::Path) {
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.len() > 4, "zip file too small: {} bytes", bytes.len());
    assert_eq!(
        &bytes[..4],
        b"PK\x03\x04",
        "file does not start with zip magic bytes"
    );
}

/// Copy input YAML to temp dir and return (input_path, base_name) for derived paths.
fn setup_temp_input(name: &str) -> std::path::PathBuf {
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(name);
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::copy(
        manifest.join(format!("{RESOURCES}/invoice.yaml")),
        &input_path,
    )
    .unwrap();
    input_path
}

#[serial_test::serial]
#[test]
fn apply_without_save_xml_does_not_create_unsigned() {
    let input_path = setup_temp_input("openubl_apply_no_unsigned.yaml");
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_apply_no_unsigned.yaml.unsigned.xml");
    let signed_path = temp_dir.join("openubl_apply_no_unsigned.yaml.signed.xml");
    let cdr_path = temp_dir.join("openubl_apply_no_unsigned.yaml.cdr.zip");

    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
    let _ = std::fs::remove_file(&cdr_path);

    let output = Command::new(BINARY)
        .args(["apply", "-f", input_path.to_str().unwrap(), "--beta"])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl apply");

    assert!(
        output.status.success(),
        "apply should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("signed_xml"),
        "output should contain signed_xml: {stdout}"
    );
    assert!(
        stdout.contains("cdr"),
        "output should contain cdr: {stdout}"
    );
    assert!(
        stdout.contains("\"unsigned_xml\": null"),
        "unsigned_xml should be null when --save-xml not used: {stdout}"
    );

    // Unsigned should NOT exist
    assert!(
        !unsigned_path.exists(),
        "unsigned XML should not exist without --save-xml"
    );
    // Signed and CDR should exist
    assert!(signed_path.exists(), "signed XML should always exist");
    assert!(cdr_path.exists(), "CDR zip should exist");
    assert_valid_xml(&signed_path);
    assert_valid_zip(&cdr_path);

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&signed_path);
    let _ = std::fs::remove_file(&cdr_path);
}

#[serial_test::serial]
#[test]
fn apply_with_save_xml_creates_unsigned() {
    let input_path = setup_temp_input("openubl_apply_with_unsigned.yaml");
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_apply_with_unsigned.yaml.unsigned.xml");
    let signed_path = temp_dir.join("openubl_apply_with_unsigned.yaml.signed.xml");
    let cdr_path = temp_dir.join("openubl_apply_with_unsigned.yaml.cdr.zip");

    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
    let _ = std::fs::remove_file(&cdr_path);

    let output = Command::new(BINARY)
        .args([
            "apply",
            "-f",
            input_path.to_str().unwrap(),
            "--beta",
            "--save-xml",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl apply");

    assert!(
        output.status.success(),
        "apply should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("unsigned_xml"),
        "output should contain unsigned_xml: {stdout}"
    );
    assert!(
        !stdout.contains("\"unsigned_xml\": null"),
        "unsigned_xml should not be null when --save-xml is used: {stdout}"
    );

    // All three files should exist
    assert!(
        unsigned_path.exists(),
        "unsigned XML should exist with --save-xml"
    );
    assert!(signed_path.exists(), "signed XML should always exist");
    assert!(cdr_path.exists(), "CDR zip should exist");

    assert_valid_xml(&unsigned_path);
    assert_valid_xml(&signed_path);
    assert_valid_zip(&cdr_path);

    // Unsigned should NOT contain ds:Signature
    let unsigned_content = std::fs::read_to_string(&unsigned_path).unwrap();
    assert!(
        !unsigned_content.contains("ds:Signature"),
        "unsigned XML should not contain ds:Signature"
    );

    // Signed should contain ds:Signature
    let signed_content = std::fs::read_to_string(&signed_path).unwrap();
    assert!(
        signed_content.contains("ds:Signature"),
        "signed XML should contain ds:Signature"
    );

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
    let _ = std::fs::remove_file(&cdr_path);
}

#[serial_test::serial]
#[test]
fn apply_with_custom_output_paths() {
    let input_path = setup_temp_input("openubl_apply_custom.yaml");
    let temp_dir = std::env::temp_dir();
    let custom_unsigned = temp_dir.join("openubl_custom_unsigned.xml");
    let custom_signed = temp_dir.join("openubl_custom_signed.xml");
    let custom_cdr = temp_dir.join("openubl_custom_cdr.zip");

    let _ = std::fs::remove_file(&custom_unsigned);
    let _ = std::fs::remove_file(&custom_signed);
    let _ = std::fs::remove_file(&custom_cdr);

    let output = Command::new(BINARY)
        .args([
            "apply",
            "-f",
            input_path.to_str().unwrap(),
            "--beta",
            "--save-xml",
            custom_unsigned.to_str().unwrap(),
            "--save-signed-xml",
            custom_signed.to_str().unwrap(),
            "-o",
            custom_cdr.to_str().unwrap(),
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl apply");

    assert!(
        output.status.success(),
        "apply should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(custom_unsigned.exists(), "custom unsigned XML should exist");
    assert!(custom_signed.exists(), "custom signed XML should exist");
    assert!(custom_cdr.exists(), "custom CDR zip should exist");

    assert_valid_xml(&custom_unsigned);
    assert_valid_xml(&custom_signed);
    assert_valid_zip(&custom_cdr);

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&custom_unsigned);
    let _ = std::fs::remove_file(&custom_signed);
    let _ = std::fs::remove_file(&custom_cdr);
}

#[test]
fn apply_dry_run_without_save_xml() {
    let input_path = setup_temp_input("openubl_apply_dryrun.yaml");
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_apply_dryrun.yaml.unsigned.xml");
    let signed_path = temp_dir.join("openubl_apply_dryrun.yaml.signed.xml");
    let cdr_path = temp_dir.join("openubl_apply_dryrun.yaml.cdr.zip");

    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
    let _ = std::fs::remove_file(&cdr_path);

    let output = Command::new(BINARY)
        .args([
            "apply",
            "-f",
            input_path.to_str().unwrap(),
            "--beta",
            "--dry-run",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl apply");

    assert!(
        output.status.success(),
        "dry-run should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("signed_xml"),
        "dry-run output should contain signed_xml: {stdout}"
    );

    // Unsigned should NOT exist (--save-xml not used)
    assert!(
        !unsigned_path.exists(),
        "unsigned XML should not exist in dry-run without --save-xml"
    );
    // Signed should exist (always saved)
    assert!(signed_path.exists(), "signed XML should exist in dry-run");
    assert_valid_xml(&signed_path);
    // CDR should NOT exist (no send in dry-run)
    assert!(!cdr_path.exists(), "CDR should not exist in dry-run");

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&signed_path);
}

#[test]
fn apply_dry_run_with_save_xml() {
    let input_path = setup_temp_input("openubl_apply_dryrun_save.yaml");
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_apply_dryrun_save.yaml.unsigned.xml");
    let signed_path = temp_dir.join("openubl_apply_dryrun_save.yaml.signed.xml");

    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);

    let output = Command::new(BINARY)
        .args([
            "apply",
            "-f",
            input_path.to_str().unwrap(),
            "--beta",
            "--dry-run",
            "--save-xml",
        ])
        .env_remove("OPENUBL_USERNAME")
        .env_remove("OPENUBL_PASSWORD")
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl apply");

    assert!(
        output.status.success(),
        "dry-run should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Both unsigned and signed should exist
    assert!(
        unsigned_path.exists(),
        "unsigned XML should exist with --save-xml in dry-run"
    );
    assert!(signed_path.exists(), "signed XML should exist in dry-run");
    assert_valid_xml(&unsigned_path);
    assert_valid_xml(&signed_path);

    let _ = std::fs::remove_file(&input_path);
    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
}
