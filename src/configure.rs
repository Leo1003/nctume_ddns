use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

const CONF_FILE: &'static str = "configure.toml";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClientConf {
    token: String,
    record_id: u64,
}

impl ClientConf {
    pub fn load() -> AppResult<Self> {
        let config_str = match fs::read_to_string(CONF_FILE) {
            Ok(string) => string,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    let def_meta = Self::default();
                    def_meta.save()?;
                    warn!("Configure file not found. A default one has been generated.");
                }
                return Err(e.into());
            }
        };
        Ok(toml::from_str(&config_str)?)
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn record_id(&self) -> u64 {
        self.record_id
    }

    pub fn save(&self) -> AppResult<()> {
        let config_str = toml::to_string_pretty(self)?;
        fs::write(CONF_FILE, config_str)?;
        Ok(())
    }
}

