include!(concat!(env!("OUT_DIR"), "/xbuilder_templates.rs"));

use std::cmp::Ordering;
use std::str::{from_utf8, FromStr};

use chrono::NaiveTime;
use rust_decimal::Decimal;
use static_files::Resource;
use tera::{Context, Error, Kwargs, State, Tera, TeraResult, Value};

use crate::xbuilder::prelude::*;

fn catalog7_taxcategory(kwargs: Kwargs, _: &State) -> TeraResult<Value> {
    let igv_tipo: String = kwargs.must_get("igv_tipo")?;
    let field: String = kwargs.must_get("field")?;

    let category = Catalog7::from_code(&igv_tipo)
        .map_err(|_| Error::message("Parameter igv_tipo is not a valid value for Catalog7"))?
        .tax_category();
    match field.as_str() {
        "code" => Ok(category.code().into()),
        "nombre" => Ok(category.nombre().into()),
        "tipo" => Ok(category.tipo().into()),
        _ => Err(Error::message("Parameter field not supported.")),
    }
}

fn parse_decimal(value: &Value) -> TeraResult<Decimal> {
    let number = value
        .as_str()
        .ok_or_else(|| Error::message("number could not be parsed to string"))?;
    Decimal::from_str(number).map_err(|err| Error::message(err.to_string()))
}

pub fn multiply100(value: &Value, _: Kwargs, _: &State) -> TeraResult<Value> {
    let result = parse_decimal(value)? * Decimal::ONE_HUNDRED;
    Ok(result.to_string().into())
}

/// Rounds to `dp` decimal places (2 by default) and pads to that scale,
/// e.g. `{{ peso | round_decimal(dp=3) }}` renders 1 as "1.000".
pub fn round_decimal(value: &Value, kwargs: Kwargs, _: &State) -> TeraResult<Value> {
    let dp: u32 = kwargs.get("dp")?.unwrap_or(2);
    let rounded = parse_decimal(value)?.round_dp(dp).trunc_with_scale(dp);
    Ok(rounded.to_string().into())
}

pub fn currency(value: &Value, _: Kwargs, _: &State) -> TeraResult<Value> {
    match value.as_str() {
        Some(number) => Ok(format!("{:.2}", number).into()),
        None => Err(Error::message("currency needs a number")),
    }
}

/// Prepares a value to be placed inside a `<![CDATA[...]]>` section.
/// CDATA content is not entity-decoded, so the value must not be HTML-escaped;
/// the only sequence that needs care is `]]>`, which would close the section.
/// Use it as `{{ value | cdata | safe }}` so Tera's autoescape doesn't run on it.
pub fn cdata(value: &Value, _: Kwargs, _: &State) -> TeraResult<Value> {
    if value.is_none() || value.is_undefined() {
        return Ok("".into());
    }
    match value.as_str() {
        Some(text) => Ok(text.replace("]]>", "]]]]><![CDATA[>").into()),
        None => Ok(value.to_string().into()),
    }
}

/// Formats a time as SUNAT expects, "HH:MM:SS", dropping fractional seconds.
pub fn format_time(value: &Value, _: Kwargs, _: &State) -> TeraResult<Value> {
    let time = value
        .as_str()
        .and_then(|text| NaiveTime::from_str(text).ok())
        .ok_or_else(|| Error::message(format!("format_time: {value} is not a time")))?;
    Ok(time.format("%H:%M:%S").to_string().into())
}

pub fn format03d(value: &Value, _: Kwargs, _: &State) -> TeraResult<Value> {
    match value.as_u64() {
        Some(number) => Ok(format!("{:03}", number).into()),
        None => Err(Error::message("format03d could not find a string")),
    }
}

/// True for a number greater than zero. A missing (null) value is false,
/// unlike a plain `{% if %}`, which is true for the string "0".
pub fn gt0(value: &Value, _: Kwargs, _: &State) -> TeraResult<bool> {
    if value.is_undefined() {
        return Err(Error::message(
            "Tester `gt0` was called on an undefined variable",
        ));
    }
    if value.is_none() {
        return Ok(false);
    }
    match value.as_str().map(Decimal::from_str) {
        Some(Ok(d)) => Ok(d > Decimal::ZERO),
        Some(Err(_)) => Err(Error::message(
            "Tester `gt0` was called on a variable that isn't a Decimal",
        )),
        None => Err(Error::message(
            "Tester `gt0` was called on a variable that isn't a number",
        )),
    }
}

pub fn credito(value: &Value, _: Kwargs, _: &State) -> TeraResult<bool> {
    match value.as_str() {
        Some("Credito") => Ok(true),
        Some("Contado") => Ok(false),
        _ => Err(Error::message(
            "Tester `credito` was called on a variable that isn't Credito or Contado",
        )),
    }
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        // Tera checks the filters and tests a template uses when it is added
        tera.register_function("catalog7_taxcategory", catalog7_taxcategory);
        tera.register_filter("multiply100", multiply100);
        tera.register_filter("round_decimal", round_decimal);
        tera.register_filter("currency", currency);
        tera.register_filter("format03d", format03d);
        tera.register_filter("format_time", format_time);
        tera.register_filter("cdata", cdata);

        tera.register_test("gt0", gt0);
        tera.register_test("credito", credito);

        let resources = generate();
        let mut resources_sorted = resources.into_iter()
            .filter(|(name, _content)| name.ends_with(".xml"))
            .collect::<Vec<(&str, Resource)>>();
        // Load the renderer files at the end to avoid dependency errors between templates
        resources_sorted.sort_by(|(a,_),(b,_)| {
            if a.starts_with("renderer") && b.starts_with("renderer") {
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            } else if a.starts_with("renderer") {
                Ordering::Greater
            } else if b.starts_with("renderer") {
                Ordering::Less
            } else {
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            }
        });
        resources_sorted.iter().for_each(|(name, content)| {
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

        // tera.autoescape_on(vec![".xml"]);
        tera
    };
}

/// Renders XML
pub trait Renderer {
    fn render(&self) -> TeraResult<String>;
}

impl Renderer for Invoice {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render("renderer/invoice.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for CreditNote {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render("renderer/credit_note.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for DebitNote {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render("renderer/debit_note.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for Reversion {
    fn render(&self) -> TeraResult<String> {
        self.0.render()
    }
}

impl Renderer for VoidedDocuments {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render(
            "renderer/voided_documents.xml",
            &Context::from_serialize(self)?,
        )
    }
}

impl Renderer for SummaryDocuments {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render(
            "renderer/summary_documents.xml",
            &Context::from_serialize(self)?,
        )
    }
}

impl Renderer for Perception {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render("renderer/perception.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for Retention {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render("renderer/retention.xml", &Context::from_serialize(self)?)
    }
}

impl Renderer for DespatchAdvice {
    fn render(&self) -> TeraResult<String> {
        TEMPLATES.render(
            "renderer/despatch_advice.xml",
            &Context::from_serialize(self)?,
        )
    }
}
