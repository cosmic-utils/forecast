use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
    pub async fn get_location_data(query: &str) -> Result<Option<Vec<Location>>, reqwest::Error> {
        let params = [("q", query)];
        
        let geocoding_ans: Vec<Location> = reqwest::Client::new()
            .get("https://geocode.maps.co/search?".to_string())
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
            
        match geocoding_ans.len() {
            0 => Ok(None),
            _ => Ok(Some(geocoding_ans)),
        }
    }
    
    pub fn get_coordinates(data: &Location) -> (f64, f64) {
    (data.lat.parse::<f64>().unwrap(), data.lon.parse::<f64>().unwrap())
}

    pub fn get_display_name(data: &Location) -> String {
        data.display_name.clone()
    }
}
