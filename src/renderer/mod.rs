use tera::{Context, Tera};

use crate::models::invoice::Invoice;

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*.xml") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

pub fn render_invoice(obj: &Invoice) -> tera::Result<String> {
    TEMPLATES.render("renderer/invoice.xml", &Context::from_serialize(obj)?)
}
