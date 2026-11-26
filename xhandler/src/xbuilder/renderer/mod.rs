include!(concat!(env!("OUT_DIR"), "/xbuilder_templates.rs"));

use std::io::Write;
use std::str::{from_utf8, FromStr};

use chrono::NaiveTime;
use rust_decimal::Decimal;
use tera::{Context, Error, Kwargs, State, Tera, TeraResult, Value};

use crate::xbuilder::prelude::*;

fn catalog7_taxcategory(args: Kwargs, _: &State) -> TeraResult<String> {
    let igv_tipo: &str = args.must_get("igv_tipo")?;
    let field: &str = args.must_get("field")?;

    let category = Catalog7::from_code(igv_tipo)
        .map_err(|_| Error::message("Parameter igv_tipo is not a valid value for Catalog7"))?
        .tax_category();
    match field {
        "code" => Ok(category.code().to_string()),
        "nombre" => Ok(category.nombre().to_string()),
        "tipo" => Ok(category.tipo().to_string()),
        _ => Err(Error::message("Parameter field not supported.")),
    }
}

fn parse_decimal(number: &str) -> TeraResult<Decimal> {
    Decimal::from_str(number).map_err(|err| Error::message(err.to_string()))
}

pub fn multiply100(value: &str, _: Kwargs, _: &State) -> TeraResult<String> {
    Ok((parse_decimal(value)? * Decimal::ONE_HUNDRED).to_string())
}

/// Rounds to `dp` decimal places (2 by default) and pads to that scale,
/// e.g. `{{ peso | round_decimal(dp=3) }}` renders 1 as "1.000".
pub fn round_decimal(value: &str, args: Kwargs, _: &State) -> TeraResult<String> {
    let dp: u32 = args.get("dp")?.unwrap_or(2);
    let rounded = parse_decimal(value)?.round_dp(dp).trunc_with_scale(dp);
    Ok(rounded.to_string())
}

pub fn currency(value: &str, _: Kwargs, _: &State) -> String {
    format!("{:.2}", value)
}

/// Prepares a value to be placed inside a `<![CDATA[...]]>` section.
/// CDATA content is not entity-decoded, so the value must not be HTML-escaped;
/// the only sequence that needs care is `]]>`, which would close the section.
/// Use it as `{{ value | cdata | safe }}` so Tera's autoescape doesn't run on it.
pub fn cdata(value: &Value, _: Kwargs, _: &State) -> String {
    match value.as_str() {
        Some(text) => text.replace("]]>", "]]]]><![CDATA[>"),
        None => value.to_string(),
    }
}

/// Formats a time as SUNAT expects, "HH:MM:SS", dropping fractional seconds.
pub fn format_time(value: &str, _: Kwargs, _: &State) -> TeraResult<String> {
    let time = NaiveTime::from_str(value)
        .map_err(|_| Error::message(format!("format_time: {value} is not a time")))?;
    Ok(time.format("%H:%M:%S").to_string())
}

pub fn format03d(value: u64, _: Kwargs, _: &State) -> String {
    format!("{:03}", value)
}

/// True for a number greater than zero. A missing (null) value is false,
/// unlike a plain `{% if %}`, which is true for the string "0".
pub fn gt0(value: &Value, _: Kwargs, _: &State) -> TeraResult<bool> {
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

/// Tera 1's escaping, kept so the rendered XML doesn't change:
/// Tera 2 no longer escapes `/` and writes `'` as `&#39;`.
pub(crate) fn escape_xml(input: &str, out: &mut dyn Write) -> std::io::Result<()> {
    for c in input.bytes() {
        match c {
            b'&' => out.write_all(b"&amp;")?,
            b'<' => out.write_all(b"&lt;")?,
            b'>' => out.write_all(b"&gt;")?,
            b'"' => out.write_all(b"&quot;")?,
            b'\'' => out.write_all(b"&#x27;")?,
            b'/' => out.write_all(b"&#x2F;")?,
            _ => out.write_all(&[c])?,
        }
    }
    Ok(())
}

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.set_escape_fn(escape_xml);

        // Registered first: templates are checked against them when added
        tera.register_function("catalog7_taxcategory", catalog7_taxcategory);
        tera.register_filter("multiply100", multiply100);
        tera.register_filter("round_decimal", round_decimal);
        tera.register_filter("currency", currency);
        tera.register_filter("format03d", format03d);
        tera.register_filter("format_time", format_time);
        tera.register_filter("cdata", cdata);

        tera.register_test("gt0", gt0);
        tera.register_test("credito", credito);

        let templates = generate()
            .into_iter()
            .filter(|(name, _)| name.ends_with(".xml"))
            .map(|(name, content)| (name, from_utf8(content.data).expect("Template is not valid UTF-8")));
        if let Err(e) = tera.add_raw_templates(templates) {
            println!("Adding template error(s): {}", e);
            ::std::process::exit(1);
        }

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
