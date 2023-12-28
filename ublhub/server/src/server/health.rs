use actix_web::{get, Responder, web};

use crate::AppState;
use crate::server::Error;

#[utoipa::path(
    responses(
        (status = 200, description = "Liveness"),
    ),
)]
#[get("/health/live")]
pub async fn liveness(
    _: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    Ok("Live")
}

#[utoipa::path(
    responses(
        (status = 200, description = "Readiness"),
    ),
)]
#[get("/health/read")]
pub async fn readiness(_: web::Data<AppState>) -> Result<impl Responder, Error> {
    Ok("Read")
}
