use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub default_screen: PluginType,
    pub schedules: Vec<Schedule>,
    pub plugin_config: PluginConfig,
    pub devices: Vec<Device>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub mac_address: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Schedule {
    pub screen: String,
    pub start_time: String,
    pub end_time: String,
    pub update_interval: u32,
    pub days: Vec<String>,
    pub plugin: PluginType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")] // Ensures YAML values match Rust enum variants
pub enum PluginType {
    GithubCommitGraph(GithubCommitGraphConfig),
    StaticImage(StaticImageData),
    Custom(CustomPlugin),
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginConfig {
    pub githubcommitgraph: Option<GithubCommitGraphConfig>,
    pub staticimage: Option<StaticImageData>,
    pub custom: Option<CustomPlugin>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticImageData {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomPlugin {
    pub name: Option<String>,
    pub template: Option<String>,
    pub plugin_code: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GithubCommitGraphConfig {
    pub username: Option<String>,
    pub api_key: Option<String>,
}
