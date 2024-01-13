use actix_4_jwt_auth::AuthenticatedUser;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder};
use anyhow::anyhow;

use openubl_api::db::Transactional;
use openubl_entity::ubl_document;
use openubl_oidc::UserClaims;
use xsender::prelude::{FromPath, UblFile};

use crate::server::Error;
use crate::AppState;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[utoipa::path(responses((status = 200, description = "Upload a file")))]
#[post("/projects/{project_id}/files")]
pub async fn upload_file(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    MultipartForm(form): MultipartForm<UploadForm>,
    user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    let project_id = path.into_inner();

    let ctx = state
        .system
        .get_project(project_id, &user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
        .ok_or(Error::BadRequest {
            status: StatusCode::NOT_FOUND,
            msg: "Project not found".to_string(),
        })?;

    let temp_file = form.files.first().ok_or(Error::BadRequest {
        status: StatusCode::BAD_REQUEST,
        msg: "No file found to be processed".to_string(),
    })?;

    // Read file
    let ubl_file = UblFile::from_path(temp_file.file.path())?;
    let file_metadata = ubl_file.metadata()?;
    let file_sha256 = ubl_file.sha256();

    // Verify file so we don't upload the same file twice
    let prev_file = ctx
        .get_document_by_ubl_params(
            &file_metadata.ruc,
            &file_metadata.document_type,
            &file_metadata.document_id,
            &file_sha256,
            Transactional::None,
        )
        .await?;

    match &prev_file {
        None => {
            // Upload to storage
            let file_path = temp_file.file.path().to_str().ok_or(Error::Any(anyhow!(
                "Could not extract the file path of the temp file"
            )))?;

            let file_id = state
                .storage
                .upload_ubl_xml(
                    ctx.project.id,
                    &file_metadata.ruc,
                    &file_metadata.document_type,
                    &file_metadata.document_id,
                    &file_sha256,
                    file_path,
                )
                .await?;

            // Create file
            let document_model = ubl_document::Model {
                id: 0,
                project_id: ctx.project.id,
                file_id,
                supplier_id: file_metadata.ruc,
                document_id: file_metadata.document_id,
                document_type: file_metadata.document_type,
                voided_document_code: file_metadata.voided_line_document_type_code,
                sha256: file_sha256,
            };

            let document = ctx
                .create_document(&document_model, Transactional::None)
                .await?;

            Ok(HttpResponse::Created().json(document))
        }
        Some(_) => Err(Error::BadRequest {
            status: StatusCode::CONFLICT,
            msg: "File already uploaded".to_string(),
        }),
    }
}
