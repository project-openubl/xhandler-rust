use anyhow::anyhow;
use encoding_rs::{Encoding, UTF_8};

/// Character encoding of an XML document: its byte order mark if present,
/// otherwise the `encoding` of its `<?xml ...?>` declaration, otherwise UTF-8.
///
/// `ISO-8859-1` resolves to windows-1252, its WHATWG superset; both are the
/// same for every printable Latin-1 character.
pub fn declared_encoding(bytes: &[u8]) -> &'static Encoding {
    if let Some((encoding, _)) = Encoding::for_bom(bytes) {
        return encoding;
    }

    xml_declaration_encoding(bytes)
        .and_then(|label| Encoding::for_label(label.as_bytes()))
        .unwrap_or(UTF_8)
}

fn xml_declaration_encoding(bytes: &[u8]) -> Option<String> {
    // The declaration is ASCII in every ASCII-compatible encoding.
    let head = &bytes[..bytes.len().min(256)];
    let head = String::from_utf8_lossy(head);
    let declaration = head.strip_prefix("<?xml")?;
    let declaration = &declaration[..declaration.find("?>")?];

    let after_keyword = &declaration[declaration.find("encoding")? + "encoding".len()..];
    let after_equals = after_keyword.trim_start().strip_prefix('=')?.trim_start();
    let quote = after_equals
        .chars()
        .next()
        .filter(|c| *c == '"' || *c == '\'')?;
    let value = &after_equals[1..];
    Some(value[..value.find(quote)?].to_string())
}

/// Decodes an XML document using its declared encoding.
pub fn decode_xml(bytes: &[u8]) -> anyhow::Result<String> {
    let encoding = declared_encoding(bytes);
    let (text, _, had_errors) = encoding.decode(bytes);
    if had_errors {
        return Err(anyhow!(
            "The XML is not valid {} (as declared by the document)",
            encoding.name()
        ));
    }
    Ok(text.into_owned())
}

/// Encodes XML text back into the encoding its declaration names, so the
/// bytes sent to SUNAT agree with the `<?xml encoding?>` they carry.
pub fn encode_xml(text: &str) -> Vec<u8> {
    let encoding = declared_encoding(text.as_bytes());
    if encoding == UTF_8 {
        return text.as_bytes().to_vec();
    }
    let (bytes, _, _) = encoding.encode(text);
    bytes.into_owned()
}

#[cfg(test)]
mod tests {
    use encoding_rs::{UTF_8, WINDOWS_1252};

    use super::{declared_encoding, decode_xml, encode_xml};

    const ISO_HEADER: &str = r#"<?xml version="1.0" encoding="ISO-8859-1" standalone="no"?>"#;

    #[test]
    fn detects_declared_encoding() {
        assert_eq!(declared_encoding(ISO_HEADER.as_bytes()), WINDOWS_1252);
        assert_eq!(
            declared_encoding(b"<?xml version='1.0' encoding = 'utf-8'?><a/>"),
            UTF_8
        );
        assert_eq!(declared_encoding(b"<?xml version=\"1.0\"?><a/>"), UTF_8);
        assert_eq!(declared_encoding(b"<a/>"), UTF_8);
        assert_eq!(declared_encoding(b"\xEF\xBB\xBF<a/>"), UTF_8);
    }

    #[test]
    fn decodes_iso_8859_1() {
        let mut bytes = format!("{ISO_HEADER}<a>").into_bytes();
        bytes.extend_from_slice(b"Pe\xF1a \xC1ngel");
        bytes.extend_from_slice(b"</a>");

        let text = decode_xml(&bytes).unwrap();
        assert!(text.ends_with("<a>Peña Ángel</a>"));
        assert_eq!(encode_xml(&text), bytes);
    }

    #[test]
    fn invalid_utf8_is_an_error() {
        assert!(decode_xml(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><a>\xF1</a>").is_err());
    }

    #[test]
    fn utf8_text_is_kept_as_is() {
        let text = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><a>Peña</a>";
        assert_eq!(encode_xml(text), text.as_bytes());
    }
}
