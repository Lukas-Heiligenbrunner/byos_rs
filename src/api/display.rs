use crate::config::types::{Config, Plugin};
use rocket::serde::json::Json;
use rocket::{get, State};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::sync::Mutex;
use utoipa::OpenApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayResponse {
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
pub async fn display(
    state: &State<ImageBuffer>,
    schedule_config: &State<Config>,
) -> Result<Json<DisplayResponse>, rocket::http::Status> {
    let plugin = schedule_config.match_plugin();
    match plugin {
        Plugin::Standard(p) => {
            println!("Rendering a standard plugin: {:?}", p);
        }
        Plugin::Custom(p) => {
            println!("Rendering a custom plugin: {:?}", p);
        }
    }

    // todo template html with selected plugin with its ruby code
    // todo render html with returned html in headless browser engine
    // todo hash html
    // todo export image to bmp
    // todo rename html including html hash

    let mut images = state.images.lock().await;

    // Rotate the buffer
    if let Some(image) = images.pop_front() {
        images.push_back(image.clone());

        Ok(Json(DisplayResponse {
            status: 0,
            image_url: format!("http://192.168.0.194:8080/api/media/{}", image),
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
