use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use std::{fs, io, path::Path};

use crate::config::CliConfig;

const CONFIG_FILE_NAME: &str = "config.json";
const PHRASE_FILE_NAME: &str = "phrase";
const HISTORY_FILE_NAME: &str = "history.txt";

pub(crate) struct CliPersistence {
    pub(crate) data_dir: String,
}

impl CliPersistence {
    pub(crate) fn get_or_create_seed(&self) -> Vec<u8> {
        let filename = Path::new(&self.data_dir).join(PHRASE_FILE_NAME);
        let mnemonic = match fs::read_to_string(filename.clone()) {
            Ok(phrase) => Mnemonic::from_phrase(phrase.as_str(), Language::English).unwrap(),
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!(
                        "Can't read from file: {}, err {e}",
                        filename.to_str().unwrap()
                    );
                }
                let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
                fs::write(filename, mnemonic.phrase()).unwrap();
                mnemonic
            }
        };
        let seed = Seed::new(&mnemonic, "");
        seed.as_bytes().to_vec()
    }

    pub(crate) fn get_or_create_config(&self) -> Result<CliConfig> {
        let filename = Path::new(&self.data_dir).join(CONFIG_FILE_NAME);
        let config: CliConfig = match fs::read(filename) {
            Ok(raw) => serde_json::from_slice(raw.as_slice()).unwrap(),
            Err(_) => {
                let config = CliConfig::default();
                self.save_config(config.clone())?;
                config
            }
        };
        Ok(config)
    }

    pub(crate) fn save_config(&self, config: CliConfig) -> Result<()> {
        let filename = Path::new(&self.data_dir).join(CONFIG_FILE_NAME);
        fs::write(filename, serde_json::to_vec(&config)?)?;
        Ok(())
    }

    pub(crate) fn history_file(&self) -> String {
        let path = Path::new(&self.data_dir).join(HISTORY_FILE_NAME);
        path.to_str().unwrap().to_string()
    }
}
