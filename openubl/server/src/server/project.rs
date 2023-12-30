use actix_web::{post, web, HttpResponse, Responder};

use openubl_api::db::Transactional;
use openubl_entity::project;

use crate::server::Error;
use crate::AppState;

#[utoipa::path(
    responses(
        (status = 200, description = "Create project"),
    ),
)]
#[post("projects")]
pub async fn create_project(
    state: web::Data<AppState>,
    json: web::Json<project::Model>,
) -> Result<impl Responder, Error> {
    let prev = state
        .system
        .get_project(&json.name, Transactional::None)
        .await?;

    match prev {
        None => {
            let ctx = state
                .system
                .create_project(&json, Transactional::None)
                .await
                .map_err(Error::System)?;
            Ok(HttpResponse::Ok().json(ctx.project))
        }
        Some(_) => Ok(HttpResponse::Conflict().body("Project name already exists")),
    }
}
