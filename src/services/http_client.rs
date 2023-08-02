use crate::CONFIG;
use reqwest::{Client, ClientBuilder};

pub fn build_http_client() -> Client {
    if CONFIG.http_client_proxy_enabled {
        Client::new()
    } else {
        ClientBuilder::no_proxy(Default::default()).build().unwrap()
    }
}
