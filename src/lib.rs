use dyn_fmt::AsStrFormatExt;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IsItUpResponse {
    pub domain: String,
    pub port: i16,
    pub status_code: i8,
    pub response_ip: Option<String>,
    pub response_code: Option<i16>,
    pub response_time: f32,
}

const URL: &str = "https://isitup.org/{}.json";

lazy_static! {
    static ref CLIENT: Client = reqwest::blocking::Client::new();
}

/// Taps into the [isitup](https://isitup.org) API 
/// to check if a domain is up or down
/// 
/// ```
/// let response = isitup::isitup("github.com".to_string());
/// assert!(response.is_ok());
/// ```
pub fn isitup(domain: String) -> Result<IsItUpResponse, reqwest::Error> {
    CLIENT
        .get(URL.format(&[domain]))
        .send()?
        .json::<IsItUpResponse>()
}
