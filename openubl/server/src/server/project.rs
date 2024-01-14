use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use openubl_api::db::{Paginated, Transactional};
use openubl_api::system::project::ProjectContext;
use openubl_entity as entity;
use openubl_oidc::UserClaims;

use crate::server::Error;
use crate::AppState;

async fn get_project_ctx(
    state: &web::Data<AppState>,
    project_id: i32,
    user: &AuthenticatedUser<UserClaims>,
    tx: Transactional<'_>,
) -> Result<ProjectContext, Error> {
    state
        .system
        .get_project(project_id, &user.claims.user_id(), tx)
        .await
        .map_err(Error::System)?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Project not found".to_string(),
        })
}

// Projects

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
    json: web::Json<entity::project::Model>,
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

#[utoipa::path(responses((status = 200, description = "Get project")))]
#[get("/projects/{project_id}")]
pub async fn get_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    Ok(HttpResponse::Ok().json(ctx.project))
}

#[utoipa::path(responses((status = 204, description = "Update project")))]
#[put("/projects/{project_id}")]
pub async fn update_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    json: web::Json<entity::project::Model>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    ctx.update(&json, Transactional::None).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(responses((status = 204, description = "Delete project")))]
#[delete("/projects/{project_id}")]
pub async fn delete_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    ctx.delete(Transactional::None).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Documents

#[utoipa::path(responses((status = 200, description = "List documents")))]
#[get("/projects/{project_id}/documents")]
pub async fn list_documents(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    paginated: web::Query<Paginated>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let result = ctx
        .list_documents(paginated.into_inner(), Transactional::None)
        .await
        .map_err(Error::System)?;

    Ok(HttpResponse::Ok()
        .append_header(("x-total", result.num_items))
        .json(
            result
                .items
                .into_iter()
                .map(|e| e.ubl_document)
                .collect::<Vec<_>>(),
        ))
}

// Credentials

#[utoipa::path(responses((status = 200, description = "List credentials")))]
#[get("/projects/{project_id}/credentials")]
pub async fn list_credentials(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let result = ctx
        .list_credentials(Transactional::None)
        .await
        .map_err(Error::System)?
        .into_iter()
        .map(|e| e.credentials)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(responses((status = 200, description = "Create credentials")))]
#[post("/projects/{project_id}/credentials")]
pub async fn create_credentials(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    json: web::Json<entity::credentials::Model>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let credentials_ctx = ctx
        .create_credentials(&json, Transactional::None)
        .await
        .map_err(Error::System)?;
    Ok(HttpResponse::Ok().json(credentials_ctx.credentials))
}

#[utoipa::path(responses((status = 200, description = "Get credentials")))]
#[get("/projects/{project_id}/credentials/{credentials_id}")]
pub async fn get_credentials(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let (project_id, credentials_id) = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let credentials_ctx = ctx
        .get_credential(credentials_id, Transactional::None)
        .await?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Project not found".to_string(),
        })?;

    Ok(HttpResponse::Ok().json(credentials_ctx.credentials))
}

#[utoipa::path(responses((status = 204, description = "Update credentials")))]
#[put("/projects/{project_id}/credentials/{credentials_id}")]
pub async fn update_credentials(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    json: web::Json<entity::credentials::Model>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let (project_id, credentials_id) = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let credentials_ctx = ctx
        .get_credential(credentials_id, Transactional::None)
        .await?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Project not found".to_string(),
        })?;

    credentials_ctx.update(&json, Transactional::None).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(responses((status = 204, description = "Delete credentials")))]
#[delete("/projects/{project_id}/credentials/{credentials_id}")]
pub async fn delete_credentials(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let (project_id, credentials_id) = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let credentials_ctx = ctx
        .get_credential(credentials_id, Transactional::None)
        .await?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Project not found".to_string(),
        })?;

    credentials_ctx.delete(Transactional::None).await?;
    Ok(HttpResponse::NoContent().finish())
}
