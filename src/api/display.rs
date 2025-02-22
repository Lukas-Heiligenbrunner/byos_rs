use crate::api::interceptors::device_infos::DeviceInfos;
use crate::api::interceptors::token::Token;
use crate::config::types::{Config, PluginType};
use crate::plugins::github_commit_graph::plugin::GithubCommitGraphPlugin;
use crate::plugins::plugin::Plugin;
use crate::plugins::static_image::plugin::StaticImagePlugin;
use crate::renderer::bmp_renderer::BmpRenderer;
use anyhow::anyhow;
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
    _t: Token,
    device_infos: DeviceInfos,
) -> Result<Json<DisplayResponse>, rocket::http::Status> {
    let (filename, update_interval) = match create_screen(schedule_config, device_infos).await {
        Ok(f) => f,
        Err(e) => {
            error!("Error creating screen: {:?}", e);
            return Err(rocket::http::Status::InternalServerError);
        }
    };

    let server_url = std::env::var("SERVER_URL").map_err(|e| {
        error!("Error getting server url: {:?}", e);
        rocket::http::Status::InternalServerError
    })?;

    Ok(Json(DisplayResponse {
        status: 0,
        image_url: format!("{}/api/media/{}", server_url, filename),
        filename,
        refresh_rate: update_interval,
        reset_firmware: false,
        update_firmware: false,
        firmware_url: None,
        special_function: "none".to_string(),
    }))
}

async fn create_screen(
    schedule_config: &Config,
    device_infos: DeviceInfos,
) -> anyhow::Result<(String, u32)> {
    let schedule = schedule_config.match_plugin();
    let plugin_configs = &schedule_config.plugin_config;

    let plugin: Box<dyn Plugin> = match schedule.plugin {
        PluginType::GithubCommitGraph(v) => {
            let token = v.api_key.unwrap_or(
                plugin_configs
                    .githubcommitgraph
                    .clone()
                    .ok_or(anyhow!("Missing Github Token"))?
                    .api_key
                    .ok_or(anyhow!("Missing Github Token"))?,
            );
            let username = v.username.unwrap_or(
                plugin_configs
                    .githubcommitgraph
                    .clone()
                    .ok_or(anyhow!("Missing Github Token"))?
                    .username
                    .ok_or(anyhow!("Missing Github Token"))?,
            );

            Box::from(GithubCommitGraphPlugin {
                username,
                api_key: token,
            })
        }
        PluginType::StaticImage(v) => {
            let path = match v.path {
                Some(p) => p,
                None => plugin_configs
                    .staticimage
                    .clone()
                    .ok_or(anyhow!("Missing path"))?
                    .path
                    .ok_or(anyhow!("Missing path"))?,
            };

            Box::from(StaticImagePlugin { path })
        }
        PluginType::Custom(v) => {
            info!(
                "Custom plugin not implemented {:?} {:?} {:?}",
                v.name, v.plugin_code, v.template
            );
            info!("{:?}", plugin_configs.custom);
            todo!("Custom plugin not implemented")
        }
    };

    let renderer = BmpRenderer::new(device_infos.width, device_infos.width);

    info!("Rendering Screen: {}", schedule.screen);
    let template = plugin
        .template()
        .await
        .map_err(|e| anyhow!("Error rendering plugin: {:?}", e))?;

    let html_digest = md5::compute(template.as_bytes());
    let filename = format!("{:x}.bmp", html_digest);
    let file_path = format!("/tmp/{}", filename);

    if Path::new(file_path.as_str()).exists() {
        info!("File already exists: {}", file_path);
        return Ok((filename, schedule.update_interval));
    }

    let bmp = plugin.render(template, &renderer).await?;

    let mut file =
        File::create(file_path.as_str()).map_err(|e| anyhow!("Error creating file: {:?}", e))?;
    file.write_all(&bmp)
        .map_err(|e| anyhow!("Error writing file: {:?}", e))?;

    Ok((filename, schedule.update_interval))
}
