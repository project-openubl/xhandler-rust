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
            match from_utf8(content.data) {
                Ok(template_raw_content) => {
                    match tera.add_raw_template(name, template_raw_content) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Adding template error(s): {}", e);
                            ::std::process::exit(1);
                        }
                    };
                },
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
        });

        tera
    };

    pub static ref FACTURA_SERIE_REGEX: Result<Regex, regex::Error> = Regex::new("^[F|f].*$");
    pub static ref BOLETA_SERIE_REGEX: Result<Regex, regex::Error> = Regex::new("^[B|f].*$");
    pub static ref GUIA_REMISION_REMITENTE_SERIE_REGEX: Result<Regex, regex::Error> = Regex::new("^[T|t].*$");
    pub static ref GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX: Result<Regex, regex::Error> = Regex::new("^[V|v].*$");
    pub static ref HTTP_CLIENT: Result<Client, reqwest::Error> = Client::builder().connection_verbose(true).build();
}
