mod bungie;
mod config;
mod dim;
mod utils;

use js_sys::Promise;
use oauth2::AuthorizationCode;
use reqwest::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{bungie::oauth::OAuth, config::Config};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> Promise {
    future_to_promise(async move {
        let config = Config::new();

        let mut client = OAuth::init_oauth_client(&config);

        // get the url via web_sys
        let href = web_sys::window().unwrap().location().href().unwrap();
        let url = Url::parse(href.as_str()).unwrap();
        if let Some(code) = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
        {
            let code = AuthorizationCode::new(code.to_string());
            let token = client.exchange_code(code).await.unwrap();

            alert(format!("Access token: {}", token).as_str());
        } else {
            let auth_url = client.generate_auth_url();

            web_sys::window()
                .unwrap()
                .open_with_url_and_target(auth_url.as_str(), "_self")
                .unwrap();
        }

        Ok(JsValue::UNDEFINED)
    })

    // .set_href(auth_url.as_str())
    // .unwrap();
    // alert(format!("Hello, {}", auth_url).as_str());
}
