use crate::api::display::*;
use crate::api::log::*;
use crate::api::media::*;
use crate::api::setup::*;
use crate::config;
use log::{error, info};
use rocket::{routes, Config, Route};
use std::collections::VecDeque;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable as _};
use utoipa_scalar::{Scalar, Servable as _};

pub fn init_api(schedule_config: config::types::Config) -> JoinHandle<()> {
    tokio::spawn(async {
        let config = Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8080,
            ..Default::default()
        };

        #[derive(OpenApi)]
        #[openapi(
            nest(
                (path = "/api", api = crate::api::display::DisplayApi, tags = ["Display"]),
                (path = "/api", api = crate::api::log::LogApi, tags = ["Log"]),
                (path = "/api", api = crate::api::media::MediaApi, tags = ["Screen"]),
                (path = "/api", api = crate::api::setup::SetupApi, tags = ["Setup"]),
            ),
            tags(
                (name = "Display", description = "Display endpoints."),
                (name = "Log", description = "Log endpoints."),
                (name = "Screen", description = "Screen endpoints."),
                (name = "Setup", description = "Setup endpoints"),
            )
        )]
        struct ApiDoc;

        let image_paths = vec![
            "logo.bmp".to_string(),
            "peter.bmp".to_string(),
            "mathmeme.bmp".to_string(),
        ];

        let rock = rocket::custom(config)
            .manage(ImageBuffer {
                images: Mutex::new(VecDeque::from(image_paths)),
            })
            .manage(schedule_config)
            .mount("/api/", build_api())
            .mount("/", Scalar::with_url("/docs", ApiDoc::openapi()))
            .mount("/", Redoc::with_url("/redoc", ApiDoc::openapi()));

        let rock = rock.launch().await;
        match rock {
            Ok(_) => info!("Rocket shut down gracefully."),
            Err(err) => error!("Rocket had an error: {}", err),
        };
    })
}

pub fn build_api() -> Vec<Route> {
    routes![setup, media, display, log_endpoint]
}
