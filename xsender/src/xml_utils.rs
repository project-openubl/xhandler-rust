use anyhow::anyhow;
use xml::reader::XmlEvent;
use xml::EventReader;

/// An element on the path from the document root to the current node.
/// Matching is done on namespace URI + local name, never on the prefix,
/// so documents are read the same regardless of the prefixes they declare.
pub struct Element {
    pub namespace: Option<String>,
    pub local_name: String,
}

impl Element {
    pub fn is(&self, namespace: &str, local_name: &str) -> bool {
        self.local_name == local_name && self.namespace.as_deref() == Some(namespace)
    }

    pub fn is_local(&self, local_name: &str) -> bool {
        self.local_name == local_name
    }
}

/// Walks an XML document and calls `on_end` every time an element closes.
///
/// `path` goes from the root element to the closing element (inclusive) and
/// `text` is the text (including CDATA) directly inside the closing element.
/// Parse errors are returned instead of being silently ignored.
pub fn walk_xml(xml: &str, mut on_end: impl FnMut(&[Element], &str)) -> anyhow::Result<()> {
    let mut path: Vec<Element> = Vec::new();
    let mut texts: Vec<String> = Vec::new();

    for event in EventReader::from_str(xml) {
        match event.map_err(|e| anyhow!("Invalid XML: {e}"))? {
            XmlEvent::StartElement { name, .. } => {
                path.push(Element {
                    namespace: name.namespace,
                    local_name: name.local_name,
                });
                texts.push(String::new());
            }
            XmlEvent::Characters(text) | XmlEvent::CData(text) => {
                if let Some(current) = texts.last_mut() {
                    current.push_str(&text);
                }
            }
            XmlEvent::EndElement { .. } => {
                let text = texts.pop().unwrap_or_default();
                on_end(&path, &text);
                path.pop();
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::walk_xml;

    #[test]
    fn text_is_scoped_to_its_element() {
        let xml =
            r#"<a xmlns:x="urn:x"><x:b>one</x:b><x:c/><x:d><![CDATA[two & three]]></x:d></a>"#;
        let mut seen = Vec::new();
        walk_xml(xml, |path, text| {
            let last = path.last().unwrap();
            seen.push((
                last.local_name.clone(),
                last.namespace.clone(),
                text.to_string(),
            ));
        })
        .unwrap();

        assert_eq!(
            seen,
            vec![
                (
                    "b".to_string(),
                    Some("urn:x".to_string()),
                    "one".to_string()
                ),
                ("c".to_string(), Some("urn:x".to_string()), "".to_string()),
                (
                    "d".to_string(),
                    Some("urn:x".to_string()),
                    "two & three".to_string()
                ),
                ("a".to_string(), None, "".to_string()),
            ]
        );
    }

    #[test]
    fn invalid_xml_is_an_error() {
        assert!(walk_xml("<a><b></a>", |_, _| {}).is_err());
    }
}
