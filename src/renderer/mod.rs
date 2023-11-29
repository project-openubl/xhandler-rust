use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::str::FromStr;

use tera::helpers::tests::{number_args_allowed, value_defined};
use tera::{from_value, to_value, Context, Error, Function, Tera, Value};

use crate::models::invoice::Invoice;
use crate::prelude::{catalog7_value_of_code, Catalog};

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

pub fn gt0(value: Option<&Value>, params: &[Value]) -> tera::Result<bool> {
    number_args_allowed("gt0", 0, params.len())?;
    value_defined("gt0", value)?;

    match value.and_then(|v| v.as_str()) {
        Some(v) => match Decimal::from_str(v) {
            Ok(d) => Ok(d > dec!(0)),
            Err(_) => Err(Error::msg(
                "Tester `gt0` was called on a variable that isn't a Decimal",
            )),
        },
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

        tera.register_tester("gt0", gt0);

        // tera.autoescape_on(vec![".xml"]);
        tera
    };
}

pub fn render_invoice(obj: &Invoice) -> tera::Result<String> {
    TEMPLATES.render("renderer/invoice.xml", &Context::from_serialize(obj)?)
}
