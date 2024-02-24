use actix_4_jwt_auth::AuthenticatedUser;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use openubl_api::db::{Paginated, Transactional};
use openubl_api::system::project::ProjectContext;
use openubl_entity as entity;
use openubl_oidc::UserClaims;

use crate::dto::{CredentialsDto, NewCredentialsDto, ProjectDto, UblDocumentDto};
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
            .into_iter()
            .map(|ctx| ProjectDto::from(ctx.project))
            .collect::<Vec<_>>(),
    ))
}

#[utoipa::path(responses((status = 200, description = "Create project")))]
#[post("/projects")]
pub async fn create_project(
    state: web::Data<AppState>,
    json: web::Json<ProjectDto>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let prev = state
        .system
        .list_projects(&user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
        .iter()
        .any(|ctx| ctx.project.name == json.name);

    let model = entity::project::Model::from(json.into_inner());
    match prev {
        false => {
            let project_ctx = state
                .system
                .create_project(&model, &user.claims.user_id(), Transactional::None)
                .await
                .map_err(Error::System)?;
            Ok(HttpResponse::Ok().json(ProjectDto::from(project_ctx.project)))
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

    Ok(HttpResponse::Ok().json(ProjectDto::from(ctx.project)))
}

#[utoipa::path(responses((status = 204, description = "Update project")))]
#[put("/projects/{project_id}")]
pub async fn update_project(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    json: web::Json<ProjectDto>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let model = entity::project::Model::from(json.into_inner());
    ctx.update(&model, Transactional::None).await?;
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
                .map(|e| UblDocumentDto::from(e.ubl_document))
                .collect::<Vec<_>>(),
        ))
}

#[utoipa::path(responses((status = 200, description = "Get document's file")))]
#[get("/projects/{project_id}/documents/{document_id}/file")]
pub async fn get_document_file(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let (project_id, document_id) = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let document_ctx = ctx
        .get_document(document_id, Transactional::None)
        .await?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Document not found".to_string(),
        })?;

    let xml_file = state
        .storage
        .download_ubl_xml(&document_ctx.ubl_document.file_id)
        .await?;

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/xml"))
        .body(xml_file))
}

// #[utoipa::path(responses((status = 200, description = "Get document's file")))]
// #[post("/projects/{project_id}/documents/{document_id}/send")]
// pub async fn send_document(
//     state: web::Data<AppState>,
//     path: web::Path<(i32, i32)>,
//     user: AuthenticatedUser<UserClaims>,
// ) -> Result<impl Responder, Error> {
//     let (project_id, document_id) = path.into_inner();
//     let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;
//
//     let document_ctx = ctx
//         .get_document(document_id, Transactional::None)
//         .await?
//         .ok_or(Error::BadRequest {
//             status: StatusCode::NOT_FOUND,
//             msg: "Document not found".to_string(),
//         })?;
//
//     let xml_file = state
//         .storage
//         .download_ubl_xml(&document_ctx.ubl_document.file_id)
//         .await?;
//
//     let credentials_ctx = ctx
//         .get_credential_for_supplier_id("", Transactional::None)
//         .await?
//         .ok_or(Error::BadRequest {
//             status: StatusCode::BAD_REQUEST,
//             msg: "There is no credentials that match the supplier id of the document".to_string(),
//         })?;
//
//     // let a = FileSender {
//     //     urls: Urls {
//     //         invoice: "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService".to_string(),
//     //         perception_retention:"https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService".to_string(),
//     //         despatch: "https://api-cpe.sunat.gob.pe/v1/contribuyente/gem".to_string(),
//     //     },
//     //     credentials: Credentials {
//     //         username: "12345678959MODDATOS".to_string(),
//     //         password: "MODDATOS".to_string(),
//     //     },
//     // };
//
//     Ok(HttpResponse::Ok()
//         .append_header(("Content-Type", "application/xml"))
//         .body(xml_file))
// }

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
        .map(|e| CredentialsDto::from(e.credentials))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(responses((status = 200, description = "Create credentials")))]
#[post("/projects/{project_id}/credentials")]
pub async fn create_credentials(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    json: web::Json<NewCredentialsDto>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();
    let ctx = get_project_ctx(&state, project_id, &user, Transactional::None).await?;

    let supplier_ids = json.supplier_ids_applied_to.clone();
    let model = entity::credentials::Model::from(json.into_inner());

    let credentials_ctx = ctx
        .create_credentials(&model, &supplier_ids, Transactional::None)
        .await
        .map_err(Error::System)?;
    Ok(HttpResponse::Ok().json(CredentialsDto::from(credentials_ctx.credentials)))
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

    Ok(HttpResponse::Ok().json(CredentialsDto::from(credentials_ctx.credentials)))
}

#[utoipa::path(responses((status = 204, description = "Update credentials")))]
#[put("/projects/{project_id}/credentials/{credentials_id}")]
pub async fn update_credentials(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    json: web::Json<NewCredentialsDto>,
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

    let model = entity::credentials::Model::from(json.into_inner());
    credentials_ctx.update(&model, Transactional::None).await?;
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
