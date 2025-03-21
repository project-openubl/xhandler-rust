use std::collections::HashMap;
use std::fmt::Debug;
use std::process::ExitCode;
use std::sync::Arc;

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use openapi::ApiDoc;
use openubl_api::system::InnerSystem;
use openubl_common::config::Database;
use openubl_storage::StorageSystem;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::server::credentials::{
    create_credentials, delete_credentials, get_credentials, list_credentials, update_credentials,
};
use crate::server::document::{get_document_file, list_documents, send_document};
use crate::server::health;
use actix_web_static_files::{deps::static_files::Resource, ResourceFiles};
use openubl_ui::{openubl_ui, UI};

mod dto;
pub mod openapi;
pub mod server;

pub struct UiResources {
    resources: HashMap<&'static str, Resource>,
}

impl UiResources {
    pub fn new(ui: &UI) -> anyhow::Result<Self> {
        Ok(Self {
            resources: openubl_ui(ui)?,
        })
    }

    pub fn resources(&self) -> HashMap<&'static str, Resource> {
        self.resources
            .iter()
            .map(|(k, v)| {
                // unfortunately, we can't just clone, but we can do it ourselves
                (
                    *k,
                    Resource {
                        data: v.data,
                        modified: v.modified,
                        mime_type: v.mime_type,
                    },
                )
            })
            .collect()
    }
}

/// Run the API server
#[derive(clap::Args, Debug)]
pub struct ServerRun {
    #[arg(short, long, env, default_value = "0.0.0.0:8080")]
    pub bind_addr: String,

    #[command(flatten)]
    pub database: Database,

    #[arg(long, env)]
    pub bootstrap: bool,

    #[command(subcommand)]
    pub storage: openubl_storage::config::Storage,
}

impl ServerRun {
    pub async fn run(self) -> anyhow::Result<ExitCode> {
        env_logger::init();

        // UI
        let ui = UI {
            version: "".to_string(),
        };
        let ui = Arc::new(UiResources::new(&ui)?);

        // Database
        let system = match self.bootstrap {
            true => InnerSystem::bootstrap(&self.database).await?,
            false => InnerSystem::with_config(&self.database).await?,
        };

        // Storage
        let storage = StorageSystem::new(&self.storage).await?;

        let app_state = Arc::new(AppState { system, storage });

        HttpServer::new(move || {
            let (app, api) = App::new()
                .wrap(Logger::default())
                .into_utoipa_app()
                //
                .openapi(ApiDoc::openapi())
                //
                .app_data(web::Data::from(app_state.clone()))
                .app_data(TempFileConfig::default())
                // q
                .service(utoipa_actix_web::scope("/q").configure(configure_q))
                // API
                .service(utoipa_actix_web::scope("/api").configure(configure_api))
                .split_for_parts();

            app
                // Swagger
                .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", api))
                .service(web::redirect("/swagger-ui", "/swagger-ui/"))
                // UI
                .service(ResourceFiles::new("/", ui.resources()).resolve_not_found_to(""))
        })
        .bind(self.bind_addr)?
        .run()
        .await?;

        Ok(ExitCode::SUCCESS)
    }
}

pub struct AppState {
    pub system: InnerSystem,
    pub storage: StorageSystem,
}

pub fn configure_q(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    // Health
    config.service(health::liveness);
    config.service(health::readiness);
}

pub fn configure_api(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    // Documents
    config.service(list_documents);
    config.service(get_document_file);
    config.service(send_document);

    // Credentials
    config.service(list_credentials);
    config.service(create_credentials);
    config.service(get_credentials);
    config.service(update_credentials);
    config.service(delete_credentials);
}
