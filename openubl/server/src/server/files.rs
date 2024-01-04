use actix_4_jwt_auth::AuthenticatedUser;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
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

    match state
        .system
        .get_project(project_id, &user.claims.user_id(), Transactional::None)
        .await
        .map_err(Error::System)?
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(ctx) => {
            match form.files.first() {
                None => Ok(HttpResponse::BadRequest().finish()),
                Some(temp_file) => {
                    // Read file
                    let ubl_file = UblFile::from_path(temp_file.file.path())?;
                    let file_metadata = ubl_file.metadata()?;

                    // Upload to storage
                    let file_path = temp_file.file.path().to_str().ok_or(Error::Any(anyhow!(
                        "Could not extract the file path of the temp file"
                    )))?;
                    let filename = temp_file
                        .file_name
                        .clone()
                        .unwrap_or("file.xml".to_string());
                    let file_id = state
                        .storage
                        .upload(ctx.project.id, file_path, &filename)
                        .await?;

                    // Create file
                    let document_model = ubl_document::Model {
                        id: 0,
                        project_id: ctx.project.id,
                        file_id,
                        ruc: file_metadata.ruc,
                        serie_numero: file_metadata.document_id,
                        tipo_documento: file_metadata.document_type,
                        baja_tipo_documento_codigo: file_metadata.voided_line_document_type_code,
                    };

                    let document = ctx
                        .create_document(&document_model, Transactional::None)
                        .await?;
                    Ok(HttpResponse::Created().json(document))
                }
            }
        }
    }
}
