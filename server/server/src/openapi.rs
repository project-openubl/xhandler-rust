use actix_web::App;
use utoipa::{
    openapi::{Info, License},
    OpenApi,
};
use utoipa_actix_web::AppExt;

use crate::{configure_api, configure_q};

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub fn default_openapi_info() -> Info {
    let mut info = Info::new("Openubl", env!("CARGO_PKG_VERSION"));
    info.description = Some("Enviar archivos XML a la SUNAT API".into());
    info.license = {
        let mut license = License::new("Apache License, Version 2.0");
        license.identifier = Some("Apache-2.0".into());
        Some(license)
    };
    info
}

pub async fn create_openapi() -> anyhow::Result<utoipa::openapi::OpenApi> {
    let (_, mut openapi) = App::new()
        .into_utoipa_app()
        .service(utoipa_actix_web::scope("/q").configure(configure_q))
        .service(utoipa_actix_web::scope("/api").configure(configure_api))
        .split_for_parts();

    openapi.info = default_openapi_info();

    Ok(openapi)
}
