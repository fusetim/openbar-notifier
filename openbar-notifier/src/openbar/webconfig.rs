//! This module cares about retrieving the Configuration of the OpenBar Instance
//! 
//! This is a little file `config.json` that is stored and served from the WebUI.
//! It contains the API endpoint and a small token that is used to authentificate a "local" order terminal.
//! This token is not really private, but it allows us to connect using a NFC card and not a Google account,
//! which is a lot easier to use for this project.

use serde::{Deserialize, Serialize};

/// The configuration as served by the OpenBar WebUI
/// 
/// Actually only the interesting fields are specified here.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    /// The API endpoint, usually something like `https://openbar.com/api/`
    pub api: String,
    /// The local token, used to authentificate a "local" order terminal.
    /// 
    /// It is then specified for some API operations using the `X-Local-Token` header.
    pub local_token: String,
}

/// Retrieve the configuration from the OpenBar instance at `base_url/config.json`.
/// 
/// This variant allows to specify a custom `reqwest::Client`, which can be useful
/// if you want to reuse an existing client with custom settings (like a proxy, or custom
/// TLS settings).
pub async fn get_config_with_client(client: &reqwest::Client, base_url: &str) -> Result<WebConfig, Box<dyn std::error::Error>> {
    let url = format!("{}/config.json", base_url);
    let resp = client.get(&url).send().await?;
    let config = resp.json::<WebConfig>().await?;
    Ok(config)
}

/// Retrieve the configuration from the OpenBar instance at `base_url/config.json`.
pub async fn get_config(base_url: &str) -> Result<WebConfig, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    get_config_with_client(&client, base_url).await
}