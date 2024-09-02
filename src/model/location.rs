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

impl Location {
    pub async fn get_location_data(query: String) -> Result<Vec<Location>, reqwest::Error> {
        let params = [("q", query)];

        let geocoding_ans: Vec<Location> = reqwest::Client::new()
            .get("https://geocode.maps.co/search?".to_string())
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(geocoding_ans)
    }
}
