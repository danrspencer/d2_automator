mod config;
mod oauth;
mod api;

use oauth2::AuthorizationCode;

use crate::config::Config;
use crate::oauth::OAuth;
use std::io::{self, Write};

const access_token: &str = "";

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = Config::new();

    api::call_destiny_api(&access_token, &config.api_key).await.unwrap();
    return;


    // Initialize OAuth2 client
    let mut client = OAuth::init_oauth_client(&config);

    // Generate the authorization URL
    let auth_url = client.generate_auth_url();
    println!("Please go to this URL and authorize the application:");
    println!("{}", auth_url);

    // Prompt for the authorization code
    println!("Enter the authorization code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();

    // Exchange the code for an access token
    match client.exchange_code(AuthorizationCode::new(code.to_string())).await {
        Ok(token) => {
            println!("Access token: {}", token);
            api::call_destiny_api(&token, &config.api_key).await.unwrap();
        },
        Err(e) => eprintln!("Error exchanging code: {}", e),
    }
}
