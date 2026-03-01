include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use regex::Regex;
use reqwest::Client;
use std::str::from_utf8;
use tera::Tera;

pub const DS: &str = "http://www.w3.org/2000/09/xmldsig#";
pub const CBC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
pub const CAC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
pub const SAC_NS: &str =
    "urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1";

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        let resources = generate();
        resources.into_iter().filter(|(name, _content)| name.ends_with(".xml")).for_each(|(name, content)| {
            let template_raw_content = from_utf8(content.data).expect("Template is not valid UTF-8");
            tera.add_raw_template(name, template_raw_content).expect("Failed to add template");
        });

        tera
    };

    pub static ref FACTURA_SERIE_REGEX: Regex = Regex::new("^[Ff].*$").expect("valid regex");
    pub static ref BOLETA_SERIE_REGEX: Regex = Regex::new("^[Bb].*$").expect("valid regex");
    pub static ref GUIA_REMISION_REMITENTE_SERIE_REGEX: Regex = Regex::new("^[Tt].*$").expect("valid regex");
    pub static ref GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX: Regex = Regex::new("^[Vv].*$").expect("valid regex");
    pub static ref HTTP_CLIENT: Client = Client::builder().connection_verbose(true).build().expect("Failed to build HTTP client");
}
