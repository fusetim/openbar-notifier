use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod accounts_api;
pub mod auth_api;
pub mod categories_api;
pub mod items_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi;
    fn auth_api(&self) -> &dyn auth_api::AuthApi;
    fn categories_api(&self) -> &dyn categories_api::CategoriesApi;
    fn items_api(&self) -> &dyn items_api::ItemsApi;
}

pub struct ApiClient {
    accounts_api: Box<dyn accounts_api::AccountsApi>,
    auth_api: Box<dyn auth_api::AuthApi>,
    categories_api: Box<dyn categories_api::CategoriesApi>,
    items_api: Box<dyn items_api::ItemsApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            accounts_api: Box::new(accounts_api::AccountsApiClient::new(configuration.clone())),
            auth_api: Box::new(auth_api::AuthApiClient::new(configuration.clone())),
            categories_api: Box::new(categories_api::CategoriesApiClient::new(configuration.clone())),
            items_api: Box::new(items_api::ItemsApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi {
        self.accounts_api.as_ref()
    }
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        self.auth_api.as_ref()
    }
    fn categories_api(&self) -> &dyn categories_api::CategoriesApi {
        self.categories_api.as_ref()
    }
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        self.items_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub accounts_api_mock: accounts_api::MockAccountsApi,
    pub auth_api_mock: auth_api::MockAuthApi,
    pub categories_api_mock: categories_api::MockCategoriesApi,
    pub items_api_mock: items_api::MockItemsApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            accounts_api_mock: accounts_api::MockAccountsApi::new(),
            auth_api_mock: auth_api::MockAuthApi::new(),
            categories_api_mock: categories_api::MockCategoriesApi::new(),
            items_api_mock: items_api::MockItemsApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi {
        &self.accounts_api_mock
    }
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        &self.auth_api_mock
    }
    fn categories_api(&self) -> &dyn categories_api::CategoriesApi {
        &self.categories_api_mock
    }
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        &self.items_api_mock
    }
}

