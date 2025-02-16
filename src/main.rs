mod api;

use std::str::FromStr;
use dotenvy::dotenv;
use env_logger::Env;
use log::LevelFilter;
use crate::api::init::init_api;

#[tokio::main]
async fn main() {
    _ = dotenv();
    init_logger();

    init_api().await.expect("Failed to start API server");
}

pub fn init_logger() {
    let env_name = "LOG_LEVEL";
    let env = Env::default()
        .filter_or(env_name, "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::builder()
        .parse_env(env)
        // increase default rocket logging to warn
        .filter_module(
            "rocket",
            LevelFilter::from_str("warn".to_string().as_str()).unwrap_or(LevelFilter::Warn),
        )
        .filter_module("hyper::proto", LevelFilter::Warn)
        .init();
}