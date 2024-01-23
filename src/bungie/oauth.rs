use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, TokenResponse, TokenUrl,
};
use reqwest::{self, Url};

use crate::config::Config;

pub struct OAuth {
    client: BasicClient,
    verifier: Option<PkceCodeVerifier>,
}

impl OAuth {
    pub fn init_oauth_client(config: &Config) -> Self {
        let auth_url = AuthUrl::new("https://www.bungie.net/en/OAuth/Authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url =
            TokenUrl::new("https://www.bungie.net/platform/app/oauth/token/".to_string())
                .expect("Invalid token endpoint URL");

        Self {
            client: BasicClient::new(
                config.bungie_client_id.clone(),
                None, // No client secret for public clients
                auth_url,
                Some(token_url),
            )
            .set_redirect_uri(
                RedirectUrl::new(config.redirect_uri.clone().to_string())
                    .expect("Invalid redirect URL"),
            ),
            verifier: None,
        }
    }

    // Function to generate the authorization URL
    pub fn generate_auth_url(&mut self) -> Url {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        self.verifier = Some(pkce_code_verifier);

        let (auth_url, _) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge.clone())
            .url();

        auth_url
    }

    // Function to exchange the code for an access token
    pub async fn exchange_code(self, code: AuthorizationCode) -> Result<String, reqwest::Error> {
        let token = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(self.verifier.unwrap())
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .unwrap();

        Ok(token.access_token().secret().clone())
    }
}
