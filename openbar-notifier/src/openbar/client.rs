use std::sync::Arc;

use openbar_api::apis::configuration::Configuration as BarConfiguration;
use openbar_api::apis::auth_api::{AuthApiClient};

/// `OpenBarClient` provides a convenient wrapper for interacting with the OpenBar API.
/// It manages API configuration, authentication tokens, and exposes API clients.
///
/// # Fields
/// - `bar_config`: Shared configuration for API requests.
///
/// # Methods
/// - `new(api_base: &str) -> Self`: Creates a new client with the given API base URL.
/// - `with_client(api_base: &str, client: reqwest::Client) -> Self`: Creates a new client with a custom HTTP client.
/// - `set_local_token(&mut self, token: &str)`: Sets the bearer access token for authentication.
/// - `as_auth(&self) -> AuthApiClient`: Returns an authentication API client using the current configuration.
#[derive(Default)]
pub struct OpenBarClient {
    bar_config: Arc<BarConfiguration>,
}

impl OpenBarClient {
    /// Create a new OpenBarClient with the specified API base URL.
    /// This uses a default reqwest client.
    pub fn new(api_base: &str) -> Self {
        return OpenBarClient::with_client(api_base, reqwest::Client::new());
    }

    /// Create a new OpenBarClient with the specified API base URL and a custom reqwest client.
    /// This can be useful if you need to customize the HTTP client (e.g., for proxies or TLS settings).
    pub fn with_client(api_base: &str, client: reqwest::Client) -> Self {
        let bar_config = BarConfiguration {
            base_path: api_base.to_string(),
            client,
            ..Default::default()
        };
        OpenBarClient {
            bar_config: Arc::new(bar_config),
            ..Default::default()
        }
    }

    /// Set the local bearer access token for authentication.
    /// This token will be included in the `X-Local-Token` header for some API requests.
    pub fn set_local_token(&mut self, token: &str) {
        let cfg = Arc::make_mut(&mut self.bar_config);
        cfg.bearer_access_token = Some(token.to_string());
    }

    /// Get an instance of the AuthApiClient using the current configuration.
    pub fn as_auth(&self) -> AuthApiClient {
        AuthApiClient::new(self.bar_config.clone())
    }
}