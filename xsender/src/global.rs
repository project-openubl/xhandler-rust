use regex::Regex;
use tera::Tera;

pub const CBC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
pub const CAC_NS: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
pub const SAC_NS: &str =
    "urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1";

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/*.xml") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };

    pub static ref FACTURA_SERIE_REGEX: Regex = Regex::new("^[F|f].*$").expect("Invalid FACTURA_SERIE_REGEX");
    pub static ref BOLETA_SERIE_REGEX: Regex = Regex::new("^[B|f].*$").expect("Invalid BOLETA_SERIE_REGEX");
    pub static ref GUIA_REMISION_REMITENTE_SERIE_REGEX: Regex = Regex::new("^[T|t].*$").expect("Invalid GUIA_REMISION_REMITENTE_SERIE_REGEX");
    pub static ref GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX: Regex = Regex::new("^[V|v].*$").expect("Invalid GUIA_REMISION_TRANSPORTISTA_SERIE_REGEX");
}
