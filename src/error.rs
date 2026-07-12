use cosmic::cosmic_config;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForecastError {
    #[error("{0}")]
    CosmicConfig(#[from] cosmic_config::Error),
    #[error("{0}")]
    Iced(#[from] cosmic::iced::Error),
}

pub type Result<T> = std::result::Result<T, ForecastError>;
