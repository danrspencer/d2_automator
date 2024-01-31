mod model;
pub mod oauth;

use reqwest;
use serde::{de::DeserializeOwned, Deserialize};
use std::error::Error;

trait EndPoint {
    fn get_url() -> String;
}

pub async fn call_destiny_api<T: DeserializeOwned + EndPoint>(
    access_token: &str,
    api_key: &str,
) -> Result<model::Response<T>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://www.bungie.net/Platform/User/{}/", T::get_url()); // Replace with the actual endpoint

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("X-API-Key", api_key) // Replace with your API key
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);

        // deserialize the response
        Ok(serde_json::from_str::<model::Response<T>>(&response_text).unwrap())
    } else {
        eprintln!("Error: {:?}", response.status());
        unimplemented!()
    }
}
