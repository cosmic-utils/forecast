use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Units {
    pub air_pressure_at_sea_level: Option<String>,
    pub air_temperature: Option<String>,
    pub cloud_area_fraction: Option<String>,
    pub precipitation_amount: Option<String>,
    pub relative_humidity: Option<String>,
    pub wind_from_direction: Option<String>,
    pub wind_speed: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub updated_at: DateTime<Utc>,
    pub units: Units,
}

#[derive(Deserialize, Serialize)]
pub struct Details {
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Summary {
    pub symbol_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct Next12Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Next1Hour {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Next6Hours {
    pub summary: Summary,
    pub details: Option<Details>,
}

#[derive(Deserialize, Serialize)]
pub struct Instant {
    pub details: Details,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub instant: Instant,
    pub next_12_hours: Option<Next12Hours>,
    pub next_1_hours: Option<Next1Hour>,
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Deserialize, Serialize)]
pub struct Timeseries {
    pub time: DateTime<Utc>,
    pub data: Data,
}

#[derive(Deserialize, Serialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Deserialize, Serialize)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
}