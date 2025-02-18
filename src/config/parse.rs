use crate::config::types::Config;
use std::fs;

pub fn parse_config() -> anyhow::Result<Config> {
    let config_str = match std::env::var("CONFIG") {
        Ok(config) => config,
        Err(_) => {
            let config_path = std::env::var("CONFIG_PATH")?;
            fs::read_to_string(config_path)?
        }
    };

    // Parse the YAML content
    let config: Config = serde_yaml::from_str(config_str.as_str())?;

    Ok(config)
}
