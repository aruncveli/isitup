use dyn_fmt::AsStrFormatExt;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct IsItUpResponse {
    pub status_code: i8,
}

pub enum DomainStatus {
    Up,
    Down,
    DoesNotExist,
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
pub fn isitup(domain: String) -> Result<DomainStatus, reqwest::Error> {
    let isitup_response = CLIENT.get(URL.format(&[domain])).send()?;
    if isitup_response.status().is_client_error() {
        return Ok(DomainStatus::DoesNotExist);
    }
    match isitup_response.json::<IsItUpResponse>() {
        Ok(valid_json) => {
            if valid_json.status_code == 1 {
                Ok(DomainStatus::Up)
            } else {
                Ok(DomainStatus::Down)
            }
        }
        Err(invalid_json) => Err(invalid_json),
    }
}

/// Check if [isitup](https://isitup.org) is accessible
///
/// ```
/// let response = isitup::ping_isitup();
/// assert!(response.is_ok());
/// ```
pub fn ping_isitup() -> Result<bool, reqwest::Error> {
    Ok(CLIENT.get(&URL[..18]).send()?.status().is_success())
}
