use std::fmt::Debug;
use std::process::ExitCode;
use std::sync::Arc;

use actix_4_jwt_auth::biscuit::{Validation, ValidationOptions};
use actix_4_jwt_auth::{Oidc, OidcBiscuitValidator, OidcConfig};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use openubl_api::system::InnerSystem;
use openubl_common::config::Database;
use openubl_oidc::config;

use crate::server::{health, project};

pub mod server;

/// Run the API server
#[derive(clap::Args, Debug)]
pub struct Run {
    #[arg(short, long, env, default_value = "[::1]:8080")]
    pub bind_addr: String,

    #[command(flatten)]
    pub database: Database,

    #[arg(long, env)]
    pub bootstrap: bool,

    #[command(flatten)]
    pub oidc: config::Oidc,
}

impl Run {
    pub async fn run(self) -> anyhow::Result<ExitCode> {
        env_logger::init();

        let system = match self.bootstrap {
            true => {
                InnerSystem::bootstrap(
                    &self.database.username,
                    &self.database.password,
                    &self.database.host,
                    self.database.port,
                    &self.database.name,
                )
                .await?
            }
            false => InnerSystem::with_config(&self.database).await?,
        };

        let app_state = Arc::new(AppState { system });

        let oidc = Oidc::new(OidcConfig::Issuer(self.oidc.auth_server_url.clone().into()))
            .await
            .unwrap();
        let oidc_validator = OidcBiscuitValidator {
            options: ValidationOptions {
                issuer: Validation::Validate(self.oidc.auth_server_url.clone()),
                ..ValidationOptions::default()
            },
        };

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::from(app_state.clone()))
                .wrap(Logger::default())
                .wrap(oidc_validator.clone())
                .app_data(oidc.clone())
                .configure(configure)
        })
        .bind(self.bind_addr)?
        .run()
        .await?;

        Ok(ExitCode::SUCCESS)
    }
}

pub struct AppState {
    pub system: InnerSystem,
}

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(health::liveness);
    config.service(health::readiness);
    config.service(project::list_projects);
    config.service(project::create_project);
    config.service(project::update_project);
    config.service(project::delete_project);
}
