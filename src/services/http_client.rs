use reqwest::{Client, ClientBuilder};
use crate::CONFIG;

pub fn build_http_client() -> Client{
    if CONFIG.http_client_proxy_enabled {
        Client::new()
    } else {
        ClientBuilder::no_proxy(Default::default()).build().unwrap()
    }
}