use std::collections::VecDeque;
use rocket::{get, State};
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::OpenApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DisplayResponse {
    status: u32,
    image_url: String,
    filename: String,
    refresh_rate: u32,
    reset_firmware: bool,
    update_firmware: bool,
    firmware_url: Option<String>,
    special_function: String,
}

#[derive(OpenApi)]
#[openapi(paths(display))]
pub struct DisplayApi;

pub struct ImageBuffer {
    pub images: Mutex<VecDeque<String>>,
}

#[utoipa::path(
    responses(
            (status = 200, description = "Todo")
    )
)]
#[get("/display")]
pub async fn display(state: &State<ImageBuffer>) -> Result<Json<DisplayResponse>, rocket::http::Status> {
    let mut images = state.images.lock().await;

    // Rotate the buffer
    if let Some(image) = images.pop_front() {
        images.push_back(image.clone());

        Ok(Json(DisplayResponse {
            status: 0,
            image_url: format!("http://192.168.0.194:8080/api/v1/media/{}", image),
            filename: image,
            refresh_rate: 60,
            reset_firmware: false,
            update_firmware: false,
            firmware_url: None,
            special_function: "none".to_string(),
        }))
    } else {
        Err(rocket::http::Status::NotFound)
    }
}