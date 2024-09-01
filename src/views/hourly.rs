use chrono::{DateTime, Utc};
use cosmic::widget::{column, text};
use cosmic::Element;

use crate::app::{App, Message};

impl App
where
    Self: cosmic::Application,
{
    pub fn view_hourly_forecast(&self) -> Element<Message> {
        let current_time = DateTime::<Utc>::from(Utc::now());
        let location = self.config.location.clone();

        let data_now = &self
            .weather_data
            .properties
            .timeseries
            .iter()
            .min_by_key(|timeseries| (timeseries.time - current_time).num_seconds().abs());

        let temp_now = if data_now.is_some() {
            data_now.unwrap().data.instant.details.air_temperature.unwrap()
        } else {
            // View is initalized before weather_data is loaded
            // Need to pass dummy data to prevent crashing
            0 as f64
        };

        column()
            .spacing(24)
            .push(text::title1(location.unwrap_or("Unknown".to_string())))
            .push(text(format!("{} degrees", temp_now)))
            .into()
    }
}
