use cosmic::widget::{column, text};
use cosmic::Element;

use crate::app::{App, Message};

impl App
where
    Self: cosmic::Application,
{
    pub fn view_hourly_forecast(&self) -> Element<Message> {
        let location = self.config.location.clone();
        let data = &self.weather_data.properties.timeseries;
        let imediate_data = if data.len() == 0 {
            // View is initalized before weather_data is loaded
            // Need to pass dummy data to prevent crashing
            0 as f64
        } else {
            data[0].data.instant.details.air_temperature.unwrap()
        };

        column()
            .spacing(24)
            .push(text::title1(location.unwrap_or("Unknown".to_string())))
            .push(text(format!("{} degrees", imediate_data)))
            .into()
    }
}
