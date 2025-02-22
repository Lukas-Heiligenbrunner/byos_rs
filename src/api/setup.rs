use crate::api::interceptors::device_id::DeviceId;
use crate::config::types::Config;
use log::warn;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
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
    message: String,
}

#[utoipa::path(
    responses(
            (status = 200, description = "Todo", body = SetupResponse)
    )
)]
#[get("/setup")]
pub async fn setup(
    id: DeviceId,
    schedule_config: &State<Config>,
) -> Result<Json<SetupResponse>, String> {
    let devices = &schedule_config.devices;

    let device = devices
        .iter()
        .find(|d| DeviceId(d.mac_address.clone()) == id)
        .ok_or(format!("Device not found. Id: {}", id.0))
        .inspect_err(|e| warn!("{}", e))?;

    Ok(Json(SetupResponse {
        status: 200,
        api_key: device.token.clone(),
        friendly_id: device.name.clone(),
        image_url: None,
        filename: None,
        message: "Setup complete".to_string(),
    }))
}
