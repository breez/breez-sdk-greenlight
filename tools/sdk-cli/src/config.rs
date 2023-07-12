use breez_sdk_core::{BreezServices, Config, EnvironmentType, GreenlightNodeConfig};
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
        let mut config = BreezServices::default_config(
            self.env.clone(),
            self.api_key.clone().unwrap_or_default(),
            breez_sdk_core::NodeConfig::Greenlight {
                config: GreenlightNodeConfig {
                    partner_credentials: None,
                    invite_code: None,
                },
            },
        );
        config.working_dir = data_dir.to_string();
        config
    }
}
