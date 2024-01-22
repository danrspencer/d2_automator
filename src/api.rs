use reqwest;
use std::error::Error;

pub async fn call_destiny_api(access_token: &str, api_key: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://www.bungie.net/Platform/Destiny/Manifest/InventoryItem/1274330687/"; // Replace with the actual endpoint

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("X-API-Key", api_key) // Replace with your API key
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        eprintln!("Error: {:?}", response.status());
    }

    Ok(())
}
