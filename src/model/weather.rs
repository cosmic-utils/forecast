use chrono::{DateTime, Local};
use cosmic::widget::{self};
use serde::{Deserialize, Serialize};

use crate::app::icon_cache::WEATHER_ICONS;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Units {
    pub air_pressure_at_sea_level: Option<String>,
    pub air_temperature: Option<String>,
    pub cloud_area_fraction: Option<String>,
    pub precipitation_amount: Option<String>,
    pub relative_humidity: Option<String>,
    pub wind_from_direction: Option<String>,
    pub wind_speed: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Meta {
    pub updated_at: DateTime<Local>,
    pub units: Units,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Details {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Summary {
    pub symbol_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Next1Hour {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Instant {
    pub details: Details,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub instant: Instant,
    pub next_12_hours: Option<Next12Hours>,
    pub next_1_hours: Option<Next1Hour>,
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timeseries {
    pub time: DateTime<Local>,
    pub data: Data,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

impl WeatherData {
    pub async fn get_weather_data(
        coords: (f64, f64),
    ) -> Result<Option<WeatherData>, reqwest::Error> {
        let query_params = [("lat", coords.0), ("lon", coords.1)];

        let weather_ans: WeatherData = reqwest::Client::new()
            .get("https://api.met.no/weatherapi/locationforecast/2.0/compact?")
            .header("User-Agent", "Weather-Cli/0.0.1")
            .query(&query_params)
            .send()
            .await?
            .json()
            .await?;

        Ok(Some(weather_ans))
    }

    pub fn icon_handle(symbol: String) -> widget::icon::Handle {
        let bytes = WEATHER_ICONS
            .get_file(format!("{symbol}.svg"))
            .map(|file| file.contents().to_vec())
            .unwrap_or_default();
        widget::icon::from_svg_bytes(bytes)
    }
}
