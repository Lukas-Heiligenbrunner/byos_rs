use std::fmt;
use log::info;
use rocket::{post};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(log_endpoint))]
pub struct LogApi;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogRespose {
    status: u32,
    message: String,
}

// Our top-level data structure now matches the JSON:
// {
//   "log": { "logs_array": [ ... ] }
// }
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogData {
    pub log: LogsArray,
}

// Implement Display for LogData using serde_json::to_string_pretty.
impl fmt::Display for LogData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(pretty_str) => write!(f, "{}", pretty_str),
            Err(e) => write!(f, "Error serializing LogData: {:?}", e),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogsArray {
    // The JSON uses the key "logs_array". We use serde's rename to map it to the field `logs`
    #[serde(rename = "logs_array")]
    pub logs: Vec<LogEntry>,
}

// Define a struct that matches one log entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogEntry {
    pub creation_timestamp: u64,
    pub device_status_stamp: DeviceStatusStamp,
    pub log_id: i32,
    pub log_message: String,
    pub log_codeline: u32,
    pub log_sourcefile: String,
    pub additional_info: AdditionalInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceStatusStamp {
    pub wifi_rssi_level: i32,
    pub wifi_status: String,
    pub refresh_rate: u32,
    pub time_since_last_sleep_start: u32,
    pub current_fw_version: String,
    pub special_function: String,
    pub battery_voltage: f64,
    pub wakeup_reason: String,
    pub free_heap_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdditionalInfo {
    pub retry_attempt: u32,
}


#[utoipa::path(
    request_body = LogData,
    responses(
            (status = 200, description = "Todo", body = LogRespose)
    )
)]
#[post("/log", data="<logs>")]
pub async fn log_endpoint(logs: Json<LogData>) -> Result<Json<LogRespose>, rocket::http::Status> {
    info!("Received logs:\n{}", *logs);

    Ok(Json(LogRespose {
        status: 200,
        message: "Log received".to_string()
    }))
}
