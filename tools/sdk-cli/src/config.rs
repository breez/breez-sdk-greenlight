use breez_sdk_core::{BreezServices, Config, EnvironmentType};
use serde::{Deserialize, Serialize};

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
