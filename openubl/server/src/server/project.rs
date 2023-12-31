use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

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
        .list_projects(&user.claims.user_id(), Transactional::None)
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
        .list_projects(&user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
        .iter()
        .any(|ctx| ctx.project.name == json.name);

    match prev {
        false => {
            let project_ctx = state
                .system
                .create_project(&json, &user.claims.user_id(), Transactional::None)
                .await
                .map_err(Error::System)?;
            Ok(HttpResponse::Ok().json(project_ctx.project))
        }
        true => Ok(HttpResponse::Conflict().body("Another project has the same name")),
    }
}

#[utoipa::path(responses((status = 204, description = "Update project")))]
#[put("/projects/{project_id}")]
pub async fn update_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    json: web::Json<project::Model>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();

    match state
        .system
        .get_project(project_id, &user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(ctx) => {
            ctx.update(&json, Transactional::None).await?;
            Ok(HttpResponse::NoContent().finish())
        }
    }
}

#[utoipa::path(responses((status = 204, description = "Delete project")))]
#[delete("/projects/{project_id}")]
pub async fn delete_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();

    match state
        .system
        .get_project(project_id, &user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(ctx) => {
            ctx.delete(Transactional::None).await?;
            Ok(HttpResponse::NoContent().finish())
        }
    }
}
