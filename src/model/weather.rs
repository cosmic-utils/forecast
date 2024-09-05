use chrono::{DateTime, Local};
use cosmic::widget::{self};
use serde::{Deserialize, Serialize};

use crate::app::{config::WeatherConfigState, icon_cache::WEATHER_ICONS};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Units {
    pub air_pressure_at_sea_level: Option<String>,
    pub air_temperature: Option<String>,
    pub air_temperature_max: Option<String>,
    pub air_temperature_min: Option<String>,
    pub cloud_area_fraction: Option<String>,
    pub precipitation_amount: Option<String>,
    pub relative_humidity: Option<String>,
    pub wind_from_direction: Option<String>,
    pub wind_speed: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Meta {
    pub updated_at: DateTime<Local>,
    pub units: Units,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Details {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub air_temperature_max: Option<f64>,
    pub air_temperature_min: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Summary {
    pub symbol_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Next1Hour {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Instant {
    pub details: Details,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Data {
    pub instant: Instant,
    pub next_12_hours: Option<Next12Hours>,
    pub next_1_hours: Option<Next1Hour>,
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Timeseries {
    pub time: DateTime<Local>,
    pub data: Data,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

impl WeatherData {
    pub async fn get_weather_data(
        coords: (f64, f64),
        last_request: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Option<(WeatherConfigState, WeatherRequestStatus)>, reqwest::Error> {
        let mut status = WeatherRequestStatus::Other;
        let query_params = [("lat", coords.0), ("lon", coords.1)];

        // note: There should not be multiple clients. The client is behind an Arc, so cloning it is cheap.
        let mut res = reqwest::Client::new()
            .get("https://api.met.no/weatherapi/locationforecast/2.0/complete?")
            .header("User-Agent", "Cosmic-Ext-Weather/0.1.0");
        if let Some(last_request) = last_request {
            let format_str = "%a, %d %b %Y %H:%M:%S GMT";
            res = res.header(
                "If-Modified-Since",
                last_request.format(format_str).to_string(),
            )
        }
        let res = res.query(&query_params).send().await?;

        let get_header_date = |key: &str| {
            res.headers().get(key).and_then(|date| {
                Some(
                    chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc2822(
                        date.to_str().ok()?,
                    )
                    .ok()?,
                )
            })
        };

        let expires = get_header_date("Expires");

        let last_request: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();

        let weather_ans = if res.status().as_u16() == 304 {
            status = WeatherRequestStatus::NotModified;
            None
        } else {
            let weather_ans: WeatherData = res.json().await?;
            Some(weather_ans)
        };

        let weather_config_state = WeatherConfigState {
            last_request: Some(last_request),
            expires,
            weather_data: weather_ans,
        };
        Ok(Some((weather_config_state, status)))
    }

    pub fn icon_handle(symbol: String) -> widget::icon::Handle {
        let bytes = WEATHER_ICONS
            .get_file(format!("{symbol}.svg"))
            .map(|file| file.contents().to_vec())
            .unwrap_or_default();
        widget::icon::from_svg_bytes(bytes)
    }
}
#[derive(Clone, Debug)]
pub enum WeatherRequestStatus {
    NotModified,
    Other,
}
