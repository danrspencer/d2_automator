use dotenv::dotenv;
use oauth2::{ClientId, ClientSecret, RedirectUrl};
use std::env;

pub struct Config {
    pub client_id: ClientId,
    pub api_key: String,
    pub redirect_uri: RedirectUrl,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables

        let config = Config {
            client_id: ClientId::new(env::var("DESTINY_CLIENT_ID").expect("Missing client ID")),
            api_key: 
                env::var("DESTINY_API_KEY").expect("Missing api key secret"),
            redirect_uri: RedirectUrl::new(
                env::var("DESTINY_REDIRECT_URI").expect("Missing redirect URI"),
            )
            .expect("Invalid redirect URI"),
        };

        println!("Client ID: {:?}", config.client_id);
        println!("Client secret: {:?}", config.api_key);
        println!("Redirect URI: {:?}", config.redirect_uri);

        config
    }
}
