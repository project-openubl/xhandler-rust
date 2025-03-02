include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use anyhow::{anyhow, bail, Context};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Serialize;
use serde_json::Value;
use static_files::resource::new_resource;
use static_files::Resource;
use std::cmp::max;
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

pub fn generate_index_html(
    ui: &UI,
    template_file: String,
    branding_file_content: String,
) -> tera::Result<String> {
    let template = template_file
        .replace("<%=", "{{")
        .replace("%>", "}}")
        .replace(
            "?? branding.application.title",
            "| default(value=branding.application.title)",
        )
        .replace(
            "?? branding.application.title",
            "| default(value=branding.application.title)",
        );

    let env_json = serde_json::to_string(&ui)?;
    let env_base64 = BASE64_STANDARD.encode(env_json.as_bytes());

    let branding: Value = serde_json::from_str(&branding_file_content)?;

    let mut context = tera::Context::new();
    context.insert("_env", &env_base64);
    context.insert("branding", &branding);

    tera::Tera::one_off(&template, &context, true)
}

pub fn openubl_ui(ui: &UI) -> anyhow::Result<HashMap<&'static str, Resource>> {
    let mut resources = generate();

    let template_file = resources.get("index.html.ejs");
    let branding_file_content = resources.get("branding/strings.json");

    let result = INDEX_HTML.get_or_init(|| {
        if let (Some(template_file), Some(branding_file_content)) =
            (template_file, branding_file_content)
        {
            let modified = max(template_file.modified, branding_file_content.modified);
            let template_file =
                from_utf8(template_file.data).context("cannot interpret template as UTF-8")?;
            let branding_file_content = from_utf8(branding_file_content.data)
                .context("cannot interpret branding as UTF-8")?;
            Ok((
                generate_index_html(
                    ui,
                    template_file.to_string(),
                    branding_file_content.to_string(),
                )
                .expect("cannot generate index.html"),
                modified,
            ))
        } else {
            bail!("Missing template or branding");
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
