use std::convert::AsRef;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    place_id: u64,
    licence: String,
    boundingbox: Vec<String>,
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    class: String,
    r#type: String,
    importance: f64,
}

impl AsRef<str> for Location {
    fn as_ref(&self) -> &str {
        &self.display_name
    }
}

impl Location {
    pub async fn get_location_data(
        query: String,
        key: String,
    ) -> Result<Vec<Location>, reqwest::Error> {
        let mut params = vec![("q", query)];

        if !key.is_empty() {
            params.push(("api_key", key));
        }

        let response = reqwest::Client::new()
            .get("https://geocode.maps.co/search?".to_string())
            .query(&params)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(resp) => resp.json::<Vec<Location>>().await,
            Err(e) => Err(e),
        }
    }
}
