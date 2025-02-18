use crate::config::types::{Config, Plugin as PluginType, StandardPluginType};
use crate::plugins::github_commit_graph::plugin::GithubCommitGraphPlugin;
use crate::plugins::Plugin;
use crate::renderer::render::render_html;
use log::{error, info};
use rocket::serde::json::Json;
use rocket::{get, State};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
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

#[utoipa::path(
    responses(
            (status = 200, description = "Todo")
    )
)]
#[get("/display")]
pub async fn display(
    schedule_config: &State<Config>,
) -> Result<Json<DisplayResponse>, rocket::http::Status> {
    let plugin_type = schedule_config.match_plugin();
    let plugin: Box<dyn Plugin> = match plugin_type {
        PluginType::Standard(p) => {
            println!("Rendering a standard plugin: {:?}", p);

            match p.plugin_type {
                StandardPluginType::GithubCommitGraph => Box::from(GithubCommitGraphPlugin {}),
                StandardPluginType::Weather => todo!("Weather"),
                StandardPluginType::News => todo!("News"),
            }
        }
        PluginType::Custom(p) => {
            println!("Rendering a custom plugin: {:?}", p);
            todo!("Custom plugin rendering")
        }
    };

    let html = plugin.render().await.map_err(|e| {
        error!("Error rendering plugin: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;

    let html_digest = md5::compute(html.as_bytes());
    let filename = format!("{:x}.bmp", html_digest);
    let file_path = format!("data/{}", filename);

    if Path::new(file_path.as_str()).exists() {
        info!("File already exists: {}", file_path);
        return Ok(Json(DisplayResponse {
            status: 0,
            image_url: format!("http://192.168.0.194:8080/api/media/{}", filename),
            filename,
            refresh_rate: 60,
            reset_firmware: false,
            update_firmware: false,
            firmware_url: None,
            special_function: "none".to_string(),
        }));
    }

    let bmp = render_html(html).map_err(|e| {
        error!("Error rendering html: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;

    let mut file = File::create(file_path.as_str()).map_err(|e| {
        error!("Error creating file: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;
    file.write_all(&bmp).map_err(|e| {
        error!("Error writing file: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;

    Ok(Json(DisplayResponse {
        status: 0,
        image_url: format!("http://192.168.0.194:8080/api/media/{}", filename),
        filename,
        refresh_rate: 60,
        reset_firmware: false,
        update_firmware: false,
        firmware_url: None,
        special_function: "none".to_string(),
    }))
}
