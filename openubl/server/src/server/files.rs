use actix_4_jwt_auth::AuthenticatedUser;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{post, Responder, web};

use openubl_oidc::UserClaims;
use xsender::prelude::{FromPath, UblFile};

use crate::AppState;
use crate::server::Error;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[utoipa::path(responses((status = 200, description = "Upload a file")))]
#[post("/projects/{project_id}/files")]
pub async fn upload_file(
    state: web::Data<AppState>,
    _path: web::Path<i32>,
    MultipartForm(form): MultipartForm<UploadForm>,
    _user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    for temp_file in form.files {
        let ubl_file = UblFile::from_path(temp_file.file.path())?;
        ubl_file.metadata()?;

        let file_path = temp_file.file.path().to_str().expect("Could not find filename");
        let filename = temp_file.file_name.unwrap_or("file.xml".to_string());
        state.storage.upload(file_path, &filename).await?;
    }

    Ok("Live")
}
