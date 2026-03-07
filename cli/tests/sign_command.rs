use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_openubl");
const RESOURCES: &str = "tests/resources";
const SIGNER_RESOURCES: &str = "../xsigner/resources/test";

/// Helper: create an unsigned invoice XML via the `create` subcommand.
fn create_invoice_xml() -> String {
    let output = Command::new(BINARY)
        .args(["create", "-f", &format!("{RESOURCES}/invoice.yaml")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl create");
    assert!(
        output.status.success(),
        "create failed: {:?}",
        output.status
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn sign_xml_with_beta_certs() {
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_test_beta_sign.xml");
    let xml = create_invoice_xml();
    std::fs::write(&unsigned_path, &xml).unwrap();

    let output = Command::new(BINARY)
        .args(["sign", "-f", unsigned_path.to_str().unwrap(), "--beta"])
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("ds:Signature"),
        "signed output should contain ds:Signature"
    );
    assert!(
        stdout.contains("ds:X509Certificate"),
        "signed output should contain ds:X509Certificate"
    );

    let _ = std::fs::remove_file(&unsigned_path);
}

#[test]
fn sign_xml_with_explicit_certs() {
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_test_explicit_sign.xml");
    let xml = create_invoice_xml();
    std::fs::write(&unsigned_path, &xml).unwrap();

    let output = Command::new(BINARY)
        .args([
            "sign",
            "-f",
            unsigned_path.to_str().unwrap(),
            "--private-key",
            &format!("{SIGNER_RESOURCES}/private.key"),
            "--certificate",
            &format!("{SIGNER_RESOURCES}/public.cer"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("ds:Signature"),
        "signed output should contain ds:Signature"
    );

    let _ = std::fs::remove_file(&unsigned_path);
}

#[test]
fn sign_xml_to_output_file() {
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_test_unsigned2.xml");
    let signed_path = temp_dir.join("openubl_test_signed.xml");
    let xml = create_invoice_xml();
    std::fs::write(&unsigned_path, &xml).unwrap();

    let output = Command::new(BINARY)
        .args([
            "sign",
            "-f",
            unsigned_path.to_str().unwrap(),
            "-o",
            signed_path.to_str().unwrap(),
            "--beta",
        ])
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(output.status.success(), "exit code: {:?}", output.status);

    assert!(
        output.stdout.is_empty(),
        "stdout should be empty when -o is used"
    );

    let content = std::fs::read_to_string(&signed_path).unwrap();
    assert!(
        content.contains("ds:Signature"),
        "file should contain ds:Signature"
    );

    let _ = std::fs::remove_file(&unsigned_path);
    let _ = std::fs::remove_file(&signed_path);
}

#[test]
fn sign_xml_from_stdin() {
    let xml = create_invoice_xml();

    let mut child = Command::new(BINARY)
        .args(["sign", "-f", "-", "--beta"])
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn openubl sign");

    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(xml.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "exit code: {:?}", output.status);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("ds:Signature"),
        "signed output from stdin should contain ds:Signature"
    );
}

#[test]
fn sign_missing_certs_without_beta_fails() {
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_test_no_certs.xml");
    let xml = create_invoice_xml();
    std::fs::write(&unsigned_path, &xml).unwrap();

    let output = Command::new(BINARY)
        .args(["sign", "-f", unsigned_path.to_str().unwrap()])
        .env_remove("OPENUBL_PRIVATE_KEY")
        .env_remove("OPENUBL_CERTIFICATE")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(
        !output.status.success(),
        "should fail without certs and without --beta"
    );

    let _ = std::fs::remove_file(&unsigned_path);
}

#[test]
fn sign_nonexistent_private_key_fails() {
    let temp_dir = std::env::temp_dir();
    let unsigned_path = temp_dir.join("openubl_test_unsigned3.xml");
    let xml = create_invoice_xml();
    std::fs::write(&unsigned_path, &xml).unwrap();

    let output = Command::new(BINARY)
        .args([
            "sign",
            "-f",
            unsigned_path.to_str().unwrap(),
            "--private-key",
            "nonexistent.key",
            "--certificate",
            &format!("{SIGNER_RESOURCES}/public.cer"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(
        !output.status.success(),
        "should fail with nonexistent private key"
    );

    let _ = std::fs::remove_file(&unsigned_path);
}

#[test]
fn sign_missing_file_arg_fails() {
    let output = Command::new(BINARY)
        .args(["sign", "--beta"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute openubl sign");

    assert!(!output.status.success(), "should fail without -f");
}
