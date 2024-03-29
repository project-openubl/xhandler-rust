use std::collections::HashMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use tera::helpers::tests::{number_args_allowed, value_defined};
use tera::{from_value, to_value, Context, Error, Function, Tera, Value};

use crate::prelude::*;

fn catalog7_taxcategory() -> impl Function {
    Box::new(
        move |args: &HashMap<String, Value>| -> Result<Value, tera::Error> {
            match (args.get("igv_tipo"), args.get("field")) {
                (Some(igv_tipo_value), Some(field_value)) => {
                    let igv_tipo = from_value::<String>(igv_tipo_value.clone())?;
                    let field = from_value::<String>(field_value.clone())?;

                    match Catalog7::from_code(&igv_tipo) {
                        Ok(catalog7) => {
                            let category = catalog7.tax_category();
                            match field.as_str() {
                                "code" => Ok(to_value(category.code()).unwrap()),
                                "nombre" => Ok(to_value(category.nombre()).unwrap()),
                                "tipo" => Ok(to_value(category.tipo()).unwrap()),
                                _ => Err("Parameter field not supported.".into()),
                            }
                        }
                        Err(_) => {
                            Err("Parameter igv_tipo is not a valid value for Catalog7".into())
                        }
                    }
                }
                _ => Err("Parameter igv_tipo or field not defined".into()),
            }
        },
    )
}

pub fn multiply100(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    match value.as_str() {
        Some(string) => {
            let decimal = Decimal::from_str(string).unwrap() * Decimal::from_str("100").unwrap();
            Ok(to_value(decimal).unwrap())
        }
        None => Err("number could not be parsed to string".into()),
    }
}

pub fn round_decimal(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    match value.as_str() {
        Some(number) => {
            let rounded = Decimal::from_str(number)
                .unwrap()
                .round_dp(2)
                .trunc_with_scale(2);
            Ok(to_value(rounded).unwrap())
        }
        None => Err("number could not be parsed to string".into()),
    }
}

pub fn currency(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    match value.as_str() {
        Some(number) => {
            let result = format!("{:.2}", number);
            Ok(to_value(result).unwrap())
        }
        None => Err("currency needs a number".into()),
    }
}

pub fn format03d(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    match value.as_u64() {
        Some(number) => {
            let result = format!("{:03}", number);
            Ok(to_value(result).unwrap())
        }
        None => Err("format03d could not find a string".into()),
    }
}

pub fn gt0(value: Option<&Value>, params: &[Value]) -> tera::Result<bool> {
    number_args_allowed("gt0", 0, params.len())?;
    value_defined("gt0", value)?;

    match value.and_then(|v| v.as_str()) {
        Some(v) => match Decimal::from_str(v) {
            Ok(d) => Ok(d > Decimal::ZERO),
            Err(_) => Err(Error::msg(
                "Tester `gt0` was called on a variable that isn't a Decimal",
            )),
        },
        _ => Err(Error::msg(
            "Tester `gt0` was called on a variable that isn't a number",
        )),
    }
}

pub fn credito(value: Option<&Value>, params: &[Value]) -> tera::Result<bool> {
    number_args_allowed("credito", 0, params.len())?;
    value_defined("credito", value)?;

    match value.and_then(|v| v.as_str()) {
        Some("Credito") => Ok(true),
        Some("Contado") => Ok(false),
        _ => Err(Error::msg(
            "Tester `gt0` was called on a variable that isn't a number",
        )),
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
        tera.register_filter("multiply100", multiply100);
        tera.register_filter("round_decimal", round_decimal);
        tera.register_filter("currency", currency);
        tera.register_filter("format03d", format03d);

        tera.register_tester("gt0", gt0);
        tera.register_tester("credito", credito);

        // tera.autoescape_on(vec![".xml"]);
        tera
    };
}

/// Renders XML
pub trait Renderer {
    fn render(&self) -> tera::Result<String>;
}

impl Renderer for Invoice {
    fn render(&self) -> tera::Result<String> {
        TEMPLATES.render("renderer/invoice.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for CreditNote {
    fn render(&self) -> tera::Result<String> {
        TEMPLATES.render("renderer/creditNote.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for DebitNote {
    fn render(&self) -> tera::Result<String> {
        TEMPLATES.render("renderer/debitNote.xml", &Context::from_serialize(self)?)
    }
}
