use js_sys::wasm_bindgen;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, TokenResponse, TokenUrl,
};
use reqwest::{self, Url};

use crate::{alert, config::Config};

pub enum OAuth {
    RedirectToAuthUrl(Url, PkceCodeVerifier),
    ExchangeCodeForToken(BasicClient, PkceCodeVerifier, AuthorizationCode),
}

impl OAuth {
    pub fn init_oauth_client(
        config: &Config,
        verifier: Option<PkceCodeVerifier>,
        href: String,
    ) -> Self {
        let url = Url::parse(href.as_str()).unwrap();
        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string());

        let auth_url = AuthUrl::new("https://www.bungie.net/en/OAuth/Authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url =
            TokenUrl::new("https://www.bungie.net/platform/app/oauth/token/".to_string())
                .expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            config.bungie_client_id.clone(),
            None, // No client secret for public clients
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.redirect_uri.clone().to_string())
                .expect("Invalid redirect URL"),
        );

        match (verifier, code) {
            (Some(verifier), Some(code)) => OAuth::ExchangeCodeForToken(
                client,
                verifier,
                AuthorizationCode::new(code.to_string()),
            ),
            _ => {
                let (auth_url, verifier) = OAuth::generate_auth_url(client.clone());
                OAuth::RedirectToAuthUrl(auth_url, verifier)
            }
        }
    }

    // Function to generate the authorization URL
    fn generate_auth_url(client: BasicClient) -> (Url, PkceCodeVerifier) {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge.clone())
            .url();

        (auth_url, pkce_code_verifier)
    }

    // Function to exchange the code for an access token
    pub async fn exchange_code(
        client: BasicClient,
        verifier: PkceCodeVerifier,
        code: AuthorizationCode,
    ) -> Result<String, reqwest::Error> {
        let request = client
            .exchange_code(code)
            .set_pkce_verifier(verifier)
            .request_async(oauth2::reqwest::async_http_client);

        alert(format!("Request about to happen").as_str());

        let result = request.await;

        alert(format!("Result: {:?}", result).as_str());
        let token = result.unwrap();

        Ok(token.access_token().secret().clone())
    }
}
