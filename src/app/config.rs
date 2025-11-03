use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    theme, Application,
};
use serde::{Deserialize, Serialize};

use crate::model::weather::WeatherData;

use super::{App, NavPage};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Units {
    Fahrenheit,
    Celsius,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum TimeFmt {
    TwelveHr,
    TwentyFourHr,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum PressureUnits {
    Hectopascal,
    Bar,
    Kilopascal,
    Psi,
    MmHg,
    Atmosphere,
}

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Serialize, Default)]
pub struct WeatherConfigState {
    /// `Expires` response header of met.no request.
    ///
    /// No new request should be sent before this date.
    /// The weather data does not change during this period.
    #[serde(default)]
    pub expires: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// Date of the last request.
    ///
    /// Used together with the `If-Modified-Since` request header.
    /// If the weather data has not changed, the response status is `304 Not Modified`.
    #[serde(default)]
    pub last_request: Option<chrono::DateTime<chrono::FixedOffset>>,

    pub weather_data: Option<WeatherData>,
}

impl WeatherConfigState {
    pub fn config_handler() -> Option<Config> {
        Config::new_state(App::APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> Self {
        match Self::config_handler() {
            Some(config_handler) => {
                Self::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    log::info!("errors loading config state: {:?}", errs);

                    config
                })
            }
            None => Self::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum SpeedUnits {
    MetersPerSecond,
    MilesPerHour,
    KilometresPerHour,
}

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WeatherConfig {
    pub location: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub units: Units,
    pub timefmt: TimeFmt,
    pub pressure_units: PressureUnits,
    pub speed_units: SpeedUnits,
    pub app_theme: AppTheme,
    pub api_key: String,
    pub default_page: NavPage,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            location: None,
            latitude: None,
            longitude: None,
            units: Units::Fahrenheit,
            timefmt: TimeFmt::TwelveHr,
            pressure_units: PressureUnits::Hectopascal,
            speed_units: SpeedUnits::MetersPerSecond,
            app_theme: AppTheme::System,
            api_key: String::default(),
            default_page: NavPage::HourlyView,
        }
    }
}

impl WeatherConfig {
    pub fn config_handler() -> Option<Config> {
        Config::new(App::APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> WeatherConfig {
        match Self::config_handler() {
            Some(config_handler) => {
                WeatherConfig::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    log::info!("errors loading config: {:?}", errs);

                    config
                })
            }
            None => WeatherConfig::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

impl AppTheme {
    pub fn theme(&self) -> theme::Theme {
        match self {
            Self::Dark => {
                let mut t = theme::system_dark();
                t.theme_type.prefer_dark(Some(true));
                t
            }
            Self::Light => {
                let mut t = theme::system_light();
                t.theme_type.prefer_dark(Some(false));
                t
            }
            Self::System => theme::system_preference(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppError {
    Location(String),
    Weather(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Location(err) => write!(f, "{}", err),
            AppError::Weather(err) => write!(f, "{}", err),
        }
    }
}
