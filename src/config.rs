use dotenv::dotenv;
use oauth2::{ClientId, RedirectUrl};
use std::env;

pub struct Config {
    pub bungie_client_id: ClientId,
    pub bungie_api_key: String,
    pub redirect_uri: RedirectUrl,
    pub dim_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables

        let config = Config {
            bungie_client_id: ClientId::new(
                env::var("DESTINY_CLIENT_ID").expect("Missing client ID"),
            ),
            bungie_api_key: env::var("DESTINY_API_KEY").expect("Missing api key secret"),
            redirect_uri: RedirectUrl::new(
                env::var("DESTINY_REDIRECT_URI").expect("Missing redirect URI"),
            )
            .expect("Invalid redirect URI"),
            dim_api_key: env::var("DIM_API_KEY").expect("Missing DIM API key"),
        };

        config
    }
}
