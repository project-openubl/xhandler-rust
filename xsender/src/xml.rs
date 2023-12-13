use std::fs;
use std::path::Path;

pub struct Xml {
    pub filename: Option<String>,
    pub file_content: String,
}

pub trait FromPath {
    fn from_path(path: &Path) -> std::io::Result<Xml>;
}

impl FromPath for Xml {
    fn from_path(path: &Path) -> std::io::Result<Xml> {
        let filename = path
            .file_name()
            .and_then(|v| v.to_str())
            .map(|v| v.to_string());
        let file_content = fs::read_to_string(path)?;
        Ok(Xml {
            filename,
            file_content,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::xml::{FromPath, Xml};

    #[test]
    fn from_file() {
        let xml_file = Xml::from_path(Path::new("resources/test/F001-1.xml"));

        assert_eq!(xml_file.unwrap().filename.unwrap(), "F001-1.xml");
    }
}
