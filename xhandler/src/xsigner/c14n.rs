//! Canonical XML 1.0 without comments (<https://www.w3.org/TR/xml-c14n>) of a
//! whole document, the form the signature digest is computed on.

use std::collections::BTreeMap;

use quick_xml::escape::unescape;
use quick_xml::events::{BytesPI, BytesStart, Event};
use quick_xml::Reader;

use crate::xsigner::SignError;

const XML_NS: &str = "http://www.w3.org/XML/1998/namespace";

/// Namespace URI by prefix (`""` is the default namespace)
type Namespaces = BTreeMap<String, String>;

/// Where the reader is, relative to the root element
#[derive(PartialEq)]
enum Root {
    Before,
    Inside,
    After,
}

/// Canonical form of `xml`: no XML declaration, comments or DTD, line breaks
/// normalized, references and CDATA sections replaced by their characters,
/// empty elements expanded, and attributes and namespace declarations sorted,
/// keeping only the namespace declarations that change the parent's scope.
///
/// Fails on malformed documents (no single root element, text outside it,
/// undeclared prefixes, repeated attributes...), so they are never signed.
pub(crate) fn canonicalize(xml: &str) -> Result<String, SignError> {
    let mut reader = Reader::from_str(xml);
    let mut output = String::with_capacity(xml.len());
    // Namespaces in scope: the document's, then one entry per open element
    let mut scopes = vec![Namespaces::new()];
    // Character data waiting for the next markup to be written
    let mut text = String::new();
    let mut position = Root::Before;

    loop {
        let event = reader.read_event()?;
        if !matches!(
            event,
            Event::Text(_) | Event::CData(_) | Event::GeneralRef(_)
        ) {
            write_text(&mut output, &text);
            text.clear();
        }

        match event {
            Event::Start(_) | Event::Empty(_) if position == Root::After => {
                return Err(invalid("The XML has more than one root element"))
            }
            Event::Start(element) => {
                write_start_tag(&mut output, &element, &mut scopes)?;
                position = Root::Inside;
            }
            Event::Empty(element) => {
                write_start_tag(&mut output, &element, &mut scopes)?;
                write_end_tag(&mut output, element.name().as_ref(), &mut scopes)?;
                if scopes.len() == 1 {
                    position = Root::After;
                }
            }
            Event::End(element) => {
                write_end_tag(&mut output, element.name().as_ref(), &mut scopes)?;
                if scopes.len() == 1 {
                    position = Root::After;
                }
            }
            Event::Text(content) if position == Root::Inside => {
                let content = content.decode().map_err(quick_xml::Error::from)?;
                if content.contains("]]>") {
                    return Err(invalid("Text can't contain ']]>'"));
                }
                let content = normalize_line_breaks(&content);
                text.push_str(&unescape(&content).map_err(quick_xml::Error::from)?);
            }
            Event::CData(content) if position == Root::Inside => {
                let content = content.decode().map_err(quick_xml::Error::from)?;
                text.push_str(&normalize_line_breaks(&content));
            }
            Event::GeneralRef(reference) if position == Root::Inside => {
                let name = reference.decode().map_err(quick_xml::Error::from)?;
                let reference = format!("&{name};");
                text.push_str(&unescape(&reference).map_err(quick_xml::Error::from)?);
            }
            // Outside the root there may only be whitespace, which is dropped
            Event::Text(content)
                if content
                    .iter()
                    .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n')) => {}
            Event::Text(_) | Event::CData(_) | Event::GeneralRef(_) => {
                return Err(invalid("The XML has text outside its root element"))
            }
            Event::PI(instruction) => {
                if position == Root::After {
                    output.push('\n');
                }
                write_processing_instruction(&mut output, &instruction)?;
                if position == Root::Before {
                    output.push('\n');
                }
            }
            Event::DocType(_) => {
                return Err(invalid("XML documents with a DOCTYPE are not supported"))
            }
            Event::Eof if position == Root::After => return Ok(output),
            Event::Eof => return Err(invalid("The XML has no root element or it is not closed")),
            Event::Comment(_) | Event::Decl(_) => {}
        }
    }
}

/// Writes the start tag with its namespace declarations (default first, then by
/// prefix) followed by its attributes (by namespace URI, then local name), and
/// opens the element's namespace scope.
fn write_start_tag(
    output: &mut String,
    element: &BytesStart,
    scopes: &mut Vec<Namespaces>,
) -> Result<(), SignError> {
    let mut scope = scopes.last().cloned().unwrap_or_default();
    let mut attributes = Vec::new();
    for attribute in element.attributes() {
        let attribute = attribute.map_err(quick_xml::Error::from)?;
        let name = utf8(attribute.key.as_ref())?.to_string();
        let value = attribute_value(utf8(&attribute.value)?)?;
        match name.strip_prefix("xmlns") {
            Some("") => {
                scope.insert(String::new(), value);
            }
            Some(prefix) if prefix.starts_with(':') => {
                let prefix = &prefix[1..];
                if value.is_empty() {
                    return Err(invalid(format!(
                        "Namespace prefix '{prefix}' can't be undeclared"
                    )));
                }
                scope.insert(prefix.to_string(), value);
            }
            _ => attributes.push((name, value)),
        }
    }

    let qualified_name = element.name();
    let name = utf8(qualified_name.as_ref())?;
    if let Some((prefix, _)) = name.split_once(':') {
        namespace(&scope, prefix)?;
    }

    let mut sorted_attributes = Vec::with_capacity(attributes.len());
    for (name, value) in attributes {
        let expanded_name = match name.split_once(':') {
            Some((prefix, local_name)) => (
                namespace(&scope, prefix)?.to_string(),
                local_name.to_string(),
            ),
            None => (String::new(), name.clone()),
        };
        sorted_attributes.push((expanded_name, name, value));
    }
    sorted_attributes.sort();
    if let Some(pair) = sorted_attributes
        .windows(2)
        .find(|pair| pair[0].0 == pair[1].0)
    {
        return Err(invalid(format!("Attribute '{}' is repeated", pair[1].1)));
    }

    output.push('<');
    output.push_str(name);
    let parent = scopes.last();
    for (prefix, uri) in &scope {
        let parent_uri = parent
            .and_then(|parent| parent.get(prefix))
            .map_or("", String::as_str);
        if prefix == "xml" || parent_uri == uri {
            continue;
        }
        let name = match prefix.as_str() {
            "" => "xmlns".to_string(),
            prefix => format!("xmlns:{prefix}"),
        };
        write_attribute(output, &name, uri);
    }
    for (_, name, value) in &sorted_attributes {
        write_attribute(output, name, value);
    }
    output.push('>');

    scopes.push(scope);
    Ok(())
}

/// Namespace URI bound to `prefix`
fn namespace<'a>(scope: &'a Namespaces, prefix: &str) -> Result<&'a str, SignError> {
    if prefix == "xml" {
        return Ok(XML_NS);
    }
    scope
        .get(prefix)
        .map(String::as_str)
        .ok_or_else(|| invalid(format!("Namespace prefix '{prefix}' is not declared")))
}

fn write_end_tag(
    output: &mut String,
    name: &[u8],
    scopes: &mut Vec<Namespaces>,
) -> Result<(), SignError> {
    output.push_str("</");
    output.push_str(utf8(name)?);
    output.push('>');
    scopes.pop();
    Ok(())
}

fn write_attribute(output: &mut String, name: &str, value: &str) {
    output.push(' ');
    output.push_str(name);
    output.push_str("=\"");
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '"' => output.push_str("&quot;"),
            '\t' => output.push_str("&#x9;"),
            '\n' => output.push_str("&#xA;"),
            '\r' => output.push_str("&#xD;"),
            character => output.push(character),
        }
    }
    output.push('"');
}

/// `<?target data?>`, with the whitespace between target and data reduced to a
/// single space, or `<?target?>` when there is no data.
fn write_processing_instruction(
    output: &mut String,
    instruction: &BytesPI,
) -> Result<(), SignError> {
    let data = normalize_line_breaks(utf8(instruction.content())?);
    let data = data.trim_start_matches([' ', '\t', '\n']);
    output.push_str("<?");
    output.push_str(utf8(instruction.target())?);
    if !data.is_empty() {
        output.push(' ');
        output.push_str(data);
    }
    output.push_str("?>");
    Ok(())
}

fn write_text(output: &mut String, text: &str) {
    for character in text.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '\r' => output.push_str("&#xD;"),
            character => output.push(character),
        }
    }
}

/// Attribute value as an XML parser reports it: line breaks normalized,
/// literal whitespace turned into spaces, then references replaced.
fn attribute_value(raw: &str) -> Result<String, SignError> {
    let spaced = normalize_line_breaks(raw).replace(['\t', '\n'], " ");
    let value = unescape(&spaced).map_err(quick_xml::Error::from)?;
    Ok(value.into_owned())
}

/// XML 1.0 end-of-line handling: `\r\n` and lone `\r` become `\n`
fn normalize_line_breaks(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn utf8(bytes: &[u8]) -> Result<&str, SignError> {
    std::str::from_utf8(bytes).map_err(|error| invalid(error.to_string()))
}

fn invalid(message: impl Into<String>) -> SignError {
    SignError::Invalid(message.into())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::canonicalize;

    /// Expected outputs were produced by libxml2, the implementation xsigner
    /// used before. To add a case, generate its `.c14n` with libxml2, e.g.
    /// `lxml.etree.tostring(tree, method="c14n", with_comments=False)`.
    #[test]
    fn canonicalize_matches_libxml2() {
        let mut checked = 0;
        for entry in fs::read_dir("tests/resources/xsigner/c14n").unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "xml") {
                let xml = fs::read_to_string(&path).unwrap();
                let expected = fs::read_to_string(path.with_extension("c14n")).unwrap();
                assert_eq!(canonicalize(&xml).unwrap(), expected, "{path:?}");
                checked += 1;
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn canonicalize_rejects_malformed_documents() {
        let malformed = [
            "<!DOCTYPE doc [<!ENTITY e \"x\">]><doc>&e;</doc>",
            "<doc p:a=\"1\"/>",
            "<p:doc/>",
            "<a><b></b>",
            "<a/><b/>",
            "<a/>x",
            "x<a/>",
            "&amp;<a/>",
            "<a/>&#65;",
            "<a><![CDATA[x]]></a><![CDATA[y]]>",
            "<a xmlns:p=\"u\" xmlns:q=\"u\" p:x=\"1\" q:x=\"2\"/>",
            "<a xmlns:p=\"\"/>",
            "<a>]]></a>",
            "",
        ];
        for xml in malformed {
            assert!(canonicalize(xml).is_err(), "{xml:?}");
        }
    }
}
