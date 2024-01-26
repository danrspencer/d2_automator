mod utils;
mod bungie;
mod config;
mod dim;

use wasm_bindgen::prelude::*;
use oauth2::AuthorizationCode;

use crate::{bungie::oauth::OAuth, config::Config};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

}

#[wasm_bindgen]
pub fn greet() {
    let config = Config::new();

    let mut client = OAuth::init_oauth_client(&config);
    let auth_url = client.generate_auth_url();

    
    web_sys::window().unwrap().open_with_url_and_target(auth_url.as_str(), "_self").unwrap();
        
        // .set_href(auth_url.as_str())
        // .unwrap();
    // alert(format!("Hello, {}", auth_url).as_str());
}
