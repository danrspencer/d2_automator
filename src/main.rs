mod config;
mod d2_client;

use oauth2::AuthorizationCode;

use crate::config::Config;
use crate::d2_client::D2Client;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = Config::new();

    // Initialize OAuth2 client
    let mut client = D2Client::init_oauth_client(&config);

    // Generate the authorization URL
    let (auth_url, _, challenge) = client.generate_auth_url();
    println!("Please go to this URL and authorize the application:");
    println!("{}", auth_url);

    // Prompt for the authorization code
    println!("Enter the authorization code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();

    // Exchange the code for an access token
    match client.exchange_code(AuthorizationCode::new(code.to_string())).await {
        Ok(token) => println!("Access token: {}", token),
        Err(e) => eprintln!("Error exchanging code: {}", e),
    }
}
