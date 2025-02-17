use rocket::get;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(setup))]
pub struct SetupApi;

#[utoipa::path(
    responses(
            (status = 200, description = "Todo")
    )
)]
#[get("/setup")]
pub async fn setup() -> Result<(), String> {
    todo!("Setup new trmnl device");
}
