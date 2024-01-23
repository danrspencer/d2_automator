mod bungie;
mod config;
mod dim;

use oauth2::AuthorizationCode;

use crate::{bungie::oauth::OAuth, config::Config};
use std::{
    io::{self},
    mem,
};

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = Config::new();

    let token = match std::fs::read_to_string("token.txt") {
        Ok(token) => Some(token),
        Err(err) => {
            println!("Error reading token from file: {}", err);

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

            match client
                .exchange_code(AuthorizationCode::new(code.to_string()))
                .await
            {
                Ok(token) => {
                    println!("Access token: {}", token);

                    // write token to file
                    std::fs::write("token.txt", token.clone()).unwrap();

                    Some(token)
                }
                Err(e) => {
                    eprintln!("Error exchanging code: {}", e);
                    None
                }
            }
        }
    }
    .unwrap();

    let response = bungie::call_destiny_api(&token, &config.bungie_api_key)
        .await
        .unwrap();

    let membership_id = response.response.primary_membership_id;
    println!("Membership ID: {}", membership_id);

    let dim_token = dim::get_dim_token(&token, &membership_id, &config.dim_api_key)
        .await
        .unwrap();
    println!("DIM token: {}", dim_token);
}
