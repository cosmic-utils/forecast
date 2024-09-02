use cosmic::app::Settings;

use crate::app::config::WeatherConfig;
use crate::app::Flags;

use super::localize::localize;

pub fn settings() -> Settings {
    localize();
    Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false)
}

pub fn flags() -> Flags {
    let (config_handler, config) = (WeatherConfig::config_handler(), WeatherConfig::config());

    Flags {
        config_handler,
        config,
    }
}
