use std::collections::HashMap;

use tera::{Context, from_value, Function, Tera, to_value, Value};

use crate::models::invoice::Invoice;
use crate::prelude::{Catalog, catalog7_value_of_code};

fn catalog7_taxcategory() -> impl Function {
    Box::new(
        move |args: &HashMap<String, Value>| -> Result<Value, tera::Error> {
            match (args.get("igv_tipo"), args.get("field")) {
                (Some(igv_tipo_value), Some(field_value)) => {
                    let igv_tipo = from_value::<String>(igv_tipo_value.clone())?;
                    let field = from_value::<String>(field_value.clone())?;

                    match catalog7_value_of_code(&igv_tipo) {
                        Some(catalog7) => {
                            let category = catalog7.tax_category();
                            match field.as_str() {
                                "code" => Ok(to_value(category.code()).unwrap()),
                                "nombre" => Ok(to_value(category.nombre()).unwrap()),
                                "tipo" => Ok(to_value(category.tipo()).unwrap()),
                                _ => Err("Parameter field not supported.".into()),
                            }
                        }
                        None => Err("Parameter igv_tipo is not a valid value for Catalog7".into()),
                    }
                }
                _ => Err("Parameter igv_tipo or field not defined".into()),
            }
        },
    )
}

pub fn currency(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    match value.as_f64() {
        Some(number) => {
            let result = format!("{:.2}", number);
            Ok(to_value(result).unwrap())
        }
        None => Err("currency needs a number".into()),
    }
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*.xml") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.register_function("catalog7_taxcategory", catalog7_taxcategory());
        tera.register_filter("currency", currency);
        // tera.autoescape_on(vec![".xml"]);
        tera
    };
}

pub fn render_invoice(obj: &Invoice) -> tera::Result<String> {
    TEMPLATES.render("renderer/invoice.xml", &Context::from_serialize(obj)?)
}
