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
#[post("/projects/files")]
pub async fn upload_file(
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
    _user: AuthenticatedUser<UserClaims>,
) -> Result<impl Responder, Error> {
    // for f in form.files {
    //     let ubl_file = UblFile::from_path(f.file.path())?;
    //     let ubl_metadata = ubl_file.metadata()?;
    //
    //     let filename = f.file.path().to_str().expect("hello");
    //     let file_id = state.storage.upload(filename).await?;
    //     // f.file.persist(path).unwrap();
    // }

    Ok("Live")
}
