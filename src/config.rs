use oauth2::{ClientId, RedirectUrl};

pub struct Config {
    pub bungie_client_id: ClientId,
    pub bungie_api_key: String,
    pub redirect_uri: RedirectUrl,
    pub dim_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        let config = Config {
            bungie_client_id: ClientId::new(env!("DESTINY_CLIENT_ID").to_string()),
            bungie_api_key: env!("DESTINY_API_KEY").to_string(),
            redirect_uri: RedirectUrl::new(env!("DESTINY_REDIRECT_URI").to_string())
                .expect("Invalid redirect URI"),
            dim_api_key: env!("DIM_API_KEY").to_string(),
        };

        config
    }
}
