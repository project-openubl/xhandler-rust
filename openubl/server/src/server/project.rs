use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::{get, post, web, HttpResponse, Responder};

use openubl_api::db::Transactional;
use openubl_entity::project;
use openubl_oidc::UserClaims;

use crate::server::Error;
use crate::AppState;

#[utoipa::path(responses((status = 200, description = "List projects")), )]
#[get("/projects")]
pub async fn list_projects(
    state: web::Data<AppState>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let projects_ctx = state
        .system
        .get_projects_by_user_id(&user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?;

    Ok(HttpResponse::Ok().json(
        projects_ctx
            .iter()
            .map(|ctx| &ctx.project)
            .collect::<Vec<_>>(),
    ))
}

#[utoipa::path(responses((status = 200, description = "Create project")))]
#[post("/projects")]
pub async fn create_project(
    state: web::Data<AppState>,
    json: web::Json<project::Model>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let prev = state
        .system
        .get_projects_by_user_id(&user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
        .iter()
        .any(|ctx| ctx.project.name == json.name);

    match prev {
        false => {
            let project_ctx = state
                .system
                .create_project(&json, &user.claims.sub, Transactional::None)
                .await
                .map_err(Error::System)?;
            Ok(HttpResponse::Ok().json(project_ctx.project))
        }
        true => Ok(HttpResponse::Conflict().body("Another project has the same name")),
    }
}
