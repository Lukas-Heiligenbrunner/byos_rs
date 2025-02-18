use rocket::fs::NamedFile;
use rocket::get;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(media))]
pub struct MediaApi;

#[utoipa::path(
    responses(
        (status = 200, description = "Returns a BMP image"),
        (status = 404, description = "File not found")
    )
)]
#[get("/media/<filename>")]
pub async fn media(filename: &str) -> Result<NamedFile, rocket::http::Status> {
    Ok(NamedFile::open(format!("/tmp/{}", filename)).await.unwrap())
}
