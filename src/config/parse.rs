use crate::config::types::Config;
use std::fs;

pub fn parse_config() -> anyhow::Result<Config> {
    let yaml_content = fs::read_to_string("data/test.yaml")?;

    // Parse the YAML content
    let config: Config = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}
