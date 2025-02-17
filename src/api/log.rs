use rocket::get;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(log_endpoint))]
pub struct LogApi;

#[utoipa::path(
    responses(
            (status = 200, description = "Todo")
    )
)]
#[get("/log")]
pub async fn log_endpoint() -> Result<(), String> {
    todo!("log from trmnl device");
}
