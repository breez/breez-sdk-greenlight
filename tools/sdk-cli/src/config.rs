use anyhow::Result;
use breez_sdk_core::{BreezServices, Config, EnvironmentType};
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CliConfig {
    pub(crate) api_key: Option<String>,
    pub(crate) env: EnvironmentType,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            api_key: None,
            env: EnvironmentType::Production,
        }
    }
}

impl CliConfig {
    pub(crate) fn to_sdk_config(&self) -> Config {
        let mut config = BreezServices::default_config(self.env.clone());
        config.api_key = self.api_key.clone();
        config
    }
}

pub(crate) fn get_or_create_config() -> Result<CliConfig> {
    let config: CliConfig = match fs::read(CONFIG_FILE_NAME) {
        Ok(raw) => serde_json::from_slice(raw.as_slice()).unwrap(),
        Err(_) => {
            let config = CliConfig::default();
            save_config(config.clone())?;
            config
        }
    };
    Ok(config)
}

pub(crate) fn save_config(config: CliConfig) -> Result<()> {
    fs::write(CONFIG_FILE_NAME, serde_json::to_vec(&config)?)?;
    Ok(())
}
