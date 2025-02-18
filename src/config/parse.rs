use crate::config::types::Config;
use std::fs;

pub fn parse_config() -> anyhow::Result<Config> {
    let config_path = std::env::var("CONFIG_PATH")?;
    let yaml_content = fs::read_to_string(config_path)?;

    // Parse the YAML content
    let config: Config = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}
