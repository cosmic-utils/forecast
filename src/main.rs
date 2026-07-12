use app::App;
use cosmic::{app::Settings, cosmic_config::Config, iced::Limits, Application};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::{
        config::{WeatherConfig, WeatherStateConfig, CONFIG_VERSION},
        localize::localizer,
        Flags,
    },
    error::{ForecastError, Result},
};

mod app;
mod error;
mod model;
mod views;

fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("cosmic_ext_forecast=info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize localizer for language support
    let localizer = localizer();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading language for App List {}", error);
    }

    // Initialize config handler for settings
    let handler = Config::new(App::APP_ID, CONFIG_VERSION)?;
    let weather_config = WeatherConfig::config(&handler);
    let weather_state_config = WeatherStateConfig::config(&handler);

    let flags = Flags {
        handler,
        weather_config,
        weather_state_config,
    };

    // Initialize settings for the application
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false)
        .size_limits(Limits::NONE.min_width(360.0).min_height(200.0));

    // Run the application
    cosmic::app::run::<App>(settings, flags).map_err(ForecastError::Iced)
}
