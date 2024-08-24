use cosmic::app::Settings;

use crate::app::config::WeatherConfig;
use crate::app::Flags;

pub fn settings() -> Settings {
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false);
    settings
}

pub fn flags() -> Flags {
    let (config_handler, config) = (WeatherConfig::config_handler(), WeatherConfig::config());

    let flags = Flags {
        config_handler,
        config,
    };
    flags
}
