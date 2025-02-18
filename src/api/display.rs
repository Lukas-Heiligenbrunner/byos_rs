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
use anyhow::anyhow;
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
    let filename = match create_screen(schedule_config).await {
        Ok(f) => f,
        Err(e) => {
            error!("Error creating screen: {:?}", e);
            return Err(rocket::http::Status::InternalServerError);
        }
    };

    let server_url = std::env::var("OAUTH_USERINFO_URI").map_err(|e| {
        error!("Error getting server url: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;

    Ok(Json(DisplayResponse {
        status: 0,
        image_url: format!("{}/api/media/{}", server_url, filename),
        filename,
        refresh_rate: 60,
        reset_firmware: false,
        update_firmware: false,
        firmware_url: None,
        special_function: "none".to_string(),
    }))
}

async fn create_screen(schedule_config: &Config) -> anyhow::Result<String> {
    let plugin_type = schedule_config.match_plugin();
    let plugin_configs = &schedule_config.plugin_config;

    let plugin: Box<dyn Plugin> = match plugin_type {
        PluginType::Standard(p) => {
            info!("Rendering a standard plugin: {:?}", p);

            match p.plugin_type {
                StandardPluginType::GithubCommitGraph => Box::from(GithubCommitGraphPlugin {config: plugin_configs.githubcommitgraph.clone().unwrap()}),
                StandardPluginType::Weather => todo!("Weather"),
                StandardPluginType::News => todo!("News"),
            }
        }
        PluginType::Custom(p) => {
            info!("Rendering a custom plugin: {:?}", p);
            todo!("Custom plugin rendering")
        }
    };

    let html = plugin.render().await.map_err(|e| anyhow!("Error rendering plugin: {:?}", e))?;

    let html_digest = md5::compute(html.as_bytes());
    let filename = format!("{:x}.bmp", html_digest);
    let file_path = format!("data/{}", filename);

    if Path::new(file_path.as_str()).exists() {
        info!("File already exists: {}", file_path);
        return Ok(filename);
    }

    let bmp = render_html(html, 1024, 614).map_err(|e|
        anyhow!("Error rendering html: {:?}", e)
    )?;

    let mut file = File::create(file_path.as_str()).map_err(|e|
        anyhow!("Error creating file: {:?}", e)
    )?;
    file.write_all(&bmp).map_err(|e|
        anyhow!("Error writing file: {:?}", e)
    )?;

    Ok(filename)
}
