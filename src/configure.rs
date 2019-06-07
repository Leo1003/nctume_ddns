use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::time::Duration;

const CONF_FILE: &'static str = "configure.toml";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConf {
    token: String,
    record_id: u64,
    update_interval: u64,
}

impl Default for ClientConf {
    fn default() -> Self {
        Self {
            token: String::new(),
            record_id: 0,
            update_interval: 5,
        }
    }
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

    pub fn interval(&self) -> Duration {
        Duration::from_secs(self.update_interval * 60)
    }

    pub fn interval_min(&self) -> u64 {
        self.update_interval
    }

    pub fn save(&self) -> AppResult<()> {
        let config_str = toml::to_string_pretty(self)?;
        fs::write(CONF_FILE, config_str)?;
        Ok(())
    }
}
