use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry},
};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Units {
    Fahrenheit,
    Celsius
}

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub location: String,
    pub units: Units,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: "Unknown".to_string(),
            units: Units::Fahrenheit,
        }
    }
}
