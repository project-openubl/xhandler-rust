use actix_web::{post, web, HttpResponse, Responder};

use crate::AppState;

#[utoipa::path(
    responses(
        (status = 200, description = "Create project"),
    ),
)]
#[post("projects")]
pub async fn create_project(_: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
