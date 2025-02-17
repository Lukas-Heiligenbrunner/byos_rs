use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub default_screen: Plugin,
    pub schedules: Vec<Schedule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Schedule {
    pub screen: String,
    pub start_time: String,
    pub end_time: String,
    pub update_interval: u32,
    pub days: Vec<String>,
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)] // This allows for different struct types in the same field
pub enum Plugin {
    Standard(StandardPlugin),
    Custom(CustomPlugin),
}

#[derive(Debug, Deserialize, Clone)]
pub struct StandardPlugin {
    pub name: String,
    pub template_view: String,
    pub content_file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomPlugin {
    pub name: String,
    pub template: String,
    pub plugin_code: String,
}
