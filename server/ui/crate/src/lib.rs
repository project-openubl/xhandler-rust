include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use anyhow::{anyhow, bail, Context};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Serialize;
use static_files::resource::new_resource;
use static_files::Resource;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::OnceLock;

#[derive(Serialize, Clone, Default)]
pub struct UI {
    #[serde(rename(serialize = "VERSION"))]
    pub version: String,
}

pub fn openubl_ui_resources() -> HashMap<&'static str, Resource> {
    let mut resources = generate();
    if let Some(index) = resources.get("index.html.ejs") {
        resources.insert(
            "index.html",
            new_resource(index.data, index.modified, "text/html"),
        );
    }

    resources
}

pub fn generate_index_html(ui: &UI, template_file: String) -> tera::Result<String> {
    let template = template_file.replace("<%=", "{{").replace("%>", "}}");

    let env_json = serde_json::to_string(&ui)?;
    let env_base64 = BASE64_STANDARD.encode(env_json.as_bytes());

    let mut context = tera::Context::new();
    context.insert("_env", &env_base64);

    tera::Tera::one_off(&template, &context, true)
}

pub fn openubl_ui(ui: &UI) -> anyhow::Result<HashMap<&'static str, Resource>> {
    let mut resources = generate();

    let template_file = resources.get("index.html.ejs");

    let result = INDEX_HTML.get_or_init(|| {
        if let Some(template_file) = template_file {
            let modified = template_file.modified;
            let template_file =
                from_utf8(template_file.data).context("cannot interpret template as UTF-8")?;
            Ok((
                generate_index_html(ui, template_file.to_string())
                    .expect("cannot generate index.html"),
                modified,
            ))
        } else {
            bail!("Missing template");
        }
    });

    let (index_html, modified) = match result {
        Ok((index_html, modified)) => (index_html.as_bytes(), *modified),
        Err(err) => return Err(anyhow!(err)),
    };

    resources.insert("", new_resource(index_html, modified, "text/html"));

    Ok(resources)
}

static INDEX_HTML: OnceLock<anyhow::Result<(String, u64)>> = OnceLock::new();
