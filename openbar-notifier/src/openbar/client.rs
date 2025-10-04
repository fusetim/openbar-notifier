use std::sync::Arc;

use openbar_api::apis::configuration::Configuration as BarConfiguration;
use openbar_api::apis::auth_api::{AuthApi, AuthApiClient, ConnectCardError, LogoutError};
use openbar_api::apis::categories_api::{CategoriesApi, CategoriesApiClient, GetCategoriesError};
use openbar_api::apis::items_api::{GetCategoryItemsError, ItemsApi, ItemsApiClient};
use openbar_api::models::{Account, Category, ConnectCardRequest, Item};
use openbar_api::apis::Error as ApiError;

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

    /// Creates a new OpenBarClient with a custom BarConfiguration.
    /// 
    /// This method is mostly useful for testing or advanced use cases where you need to
    /// customize the entire configuration.
    pub fn with_configuration(configuration: BarConfiguration) -> Self {
        OpenBarClient {
            bar_config: Arc::new(configuration),
            ..Default::default()
        }
    }

    /// Set the local bearer access token for authentication.
    /// This token will be included in the `X-Local-Token` header for some API requests.
    pub fn set_local_token(&mut self, token: &str) {
        let cfg = Arc::make_mut(&mut self.bar_config);
        let api_key = openbar_api::apis::configuration::ApiKey {
            prefix: None,
            key: token.to_string(),
        };
        cfg.api_key = Some(api_key);
    }

    /// Get an instance of the AuthApiClient using the current configuration.
    fn as_auth(&self) -> AuthApiClient {
        AuthApiClient::new(self.bar_config.clone())
    }

    /// Get an instance of the CategoriesApiClient using the current configuration.
    fn as_categories(&self) -> CategoriesApiClient {
        CategoriesApiClient::new(self.bar_config.clone())
    }

    /// Get an instance of the ItemsApiClient using the current configuration.
    fn as_items(&self) -> ItemsApiClient {
        ItemsApiClient::new(self.bar_config.clone())
    }

    /// Log in using a card ID and PIN, returning the associated Account if successful.
    /// This is a convenience method that wraps the `connect_card` method of the AuthApiClient.
    /// 
    /// Note: this method will modify the internal state of the client by setting the necessary auth token/cookies.
    pub async fn login_by_card(&self, card_id: &str, pin: &str) -> Result<Option<Account>, ApiError<ConnectCardError>> {
        let auth_api = self.as_auth();
        let auth_req = ConnectCardRequest::new(card_id.to_owned(), pin.to_owned());
        match auth_api.connect_card(Some(auth_req)).await {
            Ok(resp) => {
                if let Some(account) = resp.account {
                    Ok(Some(*account))
                } else {
                    Ok(None)
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Log out the current user by calling the `logout` method of the AuthApiClient.
    /// 
    /// Note: this method will modify the internal state of the client by clearing the auth token/cookies.
    pub async fn logout(&self) -> Result<(), ApiError<LogoutError>> {
        let auth_api = self.as_auth();
        match auth_api.logout().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Get all categories available in the OpenBar instance.
    pub async fn get_categories(&self) -> Result<Vec<Category>, ApiError<GetCategoriesError>> {
        let categories_api = self.as_categories();
        match categories_api.get_categories(None).await {
            Ok(categories) => Ok(categories),
            Err(e) => Err(e),
        }
    }

    /// Get items for a specific category by its ID.
    pub async fn get_category_items(&self, category_id: &str) -> Result<Vec<Item>, ApiError<GetCategoryItemsError>> {
        let items_api = self.as_items();
        match items_api.get_category_items(category_id, Some(0), Some(100), None).await {
            Ok(items) => Ok(items.items),
            Err(e) => Err(e),
        }
    }
}