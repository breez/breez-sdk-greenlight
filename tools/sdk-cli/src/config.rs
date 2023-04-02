use anyhow::Result;
use breez_sdk_core::{BreezServices, Config, EnvironmentType};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    pub(crate) fn to_sdk_config(&self, data_dir: &str) -> Config {
        let mut config = BreezServices::default_config(self.env.clone());
        config.api_key = self.api_key.clone();
        config.working_dir = data_dir.to_string();
        config
    }
}

pub(crate) fn get_or_create_config(data_dir: &String) -> Result<CliConfig> {
    let filename = Path::new(data_dir).join(CONFIG_FILE_NAME);
    let config: CliConfig = match fs::read(filename) {
        Ok(raw) => serde_json::from_slice(raw.as_slice()).unwrap(),
        Err(_) => {
            let config = CliConfig::default();
            save_config(data_dir, config.clone())?;
            config
        }
    };
    Ok(config)
}

pub(crate) fn save_config(data_dir: &String, config: CliConfig) -> Result<()> {
    let filename = Path::new(data_dir).join(CONFIG_FILE_NAME);
    fs::write(filename, serde_json::to_vec(&config)?)?;
    Ok(())
}
