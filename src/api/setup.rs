use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(setup))]
pub struct SetupApi;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SetupResponse {
    status: u32,
    api_key: String,
    friendly_id: String,
    image_url: Option<String>,
    filename: Option<String>,
    message: String
}

#[utoipa::path(
    responses(
            (status = 200, description = "Todo", body = SetupResponse)
    )
)]
#[get("/setup")]
pub async fn setup() -> Result<Json<SetupResponse>, String> {
    Ok(Json(SetupResponse {
        status: 200,
        api_key: "42424242".to_string(),
        friendly_id: "myTrmnl".to_string(),
        image_url: None,
        filename: None,
        message: "Setup complete".to_string()
    }))
}
