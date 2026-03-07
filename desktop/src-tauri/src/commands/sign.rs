use xhandler::prelude::*;

const BETA_PRIVATE_KEY: &str = include_str!("../../../../xsigner/resources/test/private.key");
const BETA_CERTIFICATE: &str = include_str!("../../../../xsigner/resources/test/public.cer");

#[tauri::command]
pub fn sign_xml(
    xml_content: String,
    private_key_pem: Option<String>,
    certificate_pem: Option<String>,
    beta: bool,
) -> Result<String, String> {
    let private_key = match private_key_pem {
        Some(key) => key,
        None if beta => BETA_PRIVATE_KEY.to_string(),
        None => return Err("Se requiere la llave privada".to_string()),
    };
    let certificate = match certificate_pem {
        Some(cert) => cert,
        None if beta => BETA_CERTIFICATE.to_string(),
        None => return Err("Se requiere el certificado".to_string()),
    };

    let key_pair = RsaKeyPair::from_pkcs1_pem_and_certificate(&private_key, &certificate)
        .map_err(|e| format!("Error al leer certificados: {e}"))?;

    let signer = XSigner {
        xml_document: xml_content,
    };
    let signed_bytes = signer
        .sign(&key_pair)
        .map_err(|e| format!("Error al firmar: {e}"))?;

    String::from_utf8(signed_bytes).map_err(|e| format!("Error en XML firmado: {e}"))
}
