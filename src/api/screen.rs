use rocket::{get, Response};
use rocket::fs::NamedFile;
use rocket::response::Responder;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(generate_screen, media))]
pub struct ScreenApi;

#[utoipa::path(
    responses(
            (status = 200, description = "Todo")
    )
)]
#[get("/v1/generate_screen")]
pub async fn generate_screen() -> Result<(), String> {
    Ok(())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Returns a BMP image"),
        (status = 404, description = "File not found")
    )
)]
#[get("/v1/media/<filename>")]
pub async fn media(filename: &str) -> Result<NamedFile, rocket::http::Status> {
    Ok(NamedFile::open(format!("data/{}", filename)).await.unwrap())
}