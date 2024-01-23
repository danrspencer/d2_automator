use reqwest;
use serde_json::json;
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

pub async fn get_dim_token(
    bungie_access_token: &str,
    membership_id: &str,
    dim_api_key: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.destinyitemmanager.com/auth/token";

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-API-Key", dim_api_key)
        .body(
            json!({
                "bungieAccessToken": bungie_access_token,
                "membershipId": membership_id
            })
            .to_string(),
        )
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        Ok(response_text)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get DIM token: ".to_string()
                + &response.status().to_string()
                + &response.text().await?,
        )))
    }
}
