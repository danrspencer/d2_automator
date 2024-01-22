use dotenv::dotenv;
use oauth2::{ClientId, ClientSecret, RedirectUrl};
use std::env;

pub struct Config {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub redirect_uri: RedirectUrl,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables

        let config = Config {
            client_id: ClientId::new(env::var("DESTINY_CLIENT_ID").expect("Missing client ID")),
            client_secret: ClientSecret::new(
                env::var("DESTINY_CLIENT_SECRET").expect("Missing client secret"),
            ),
            redirect_uri: RedirectUrl::new(
                env::var("DESTINY_REDIRECT_URI").expect("Missing redirect URI"),
            )
            .expect("Invalid redirect URI"),
        };

        println!("Client ID: {:?}", config.client_id);
        println!("Client secret: {:?}", config.client_secret);
        println!("Redirect URI: {:?}", config.redirect_uri);

        config
    }
}
