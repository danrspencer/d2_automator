mod bungie;
mod config;
mod dim;
mod utils;

use js_sys::Promise;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, TokenResponse, TokenUrl,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{bungie::oauth::OAuth, config::Config};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

const PKCE_CODE_VERIFIER_KEY: &str = "pkce_code_verifier";

#[wasm_bindgen]
pub fn main() -> Promise {
    future_to_promise(async move {
        let config = Config::new();
        let window = web_sys::window().unwrap();
        let session_storage = window.session_storage().unwrap().unwrap();

        let href = window.location().href().unwrap();
        let verifier = session_storage
            .get_item(PKCE_CODE_VERIFIER_KEY)
            .unwrap()
            .map(|verifier| PkceCodeVerifier::new(verifier));
        let client = OAuth::init_oauth_client(&config, verifier, href);

        let d2_token = match client {
            OAuth::RedirectToAuthUrl(auth_url, verifier) => {
                session_storage
                    .set_item(PKCE_CODE_VERIFIER_KEY, verifier.secret())
                    .and_then(|_| window.open_with_url_and_target(auth_url.as_str(), "_self"))
                    .unwrap();
                return Ok(JsValue::UNDEFINED);
            }
            OAuth::ExchangeCodeForToken(client, verifier, code) => {
                OAuth::exchange_code(client, verifier, code).await
            }
        }
        .unwrap();

        let response = bungie::call_destiny_api(&d2_token, &config.bungie_api_key)
            .await
            .unwrap();

        let membership_id = response.response.bungie_net_user.membership_id;
        let dim_token = dim::get_dim_token(&d2_token, &membership_id, &config.dim_api_key)
            .await
            .unwrap();

        // TODO - deal with token expiration

        Ok(JsValue::UNDEFINED)
    })
}
