use std::sync::Arc;

use log::{info, error, debug};
use openbar_notifier::openbar::{webconfig::{get_config_with_client}, OpenBarClient};
use openbar_api::{apis::{auth_api::AuthApi, categories_api::CategoriesApi, items_api::ItemsApi}, models::{ConnectCardRequest, ItemState}};

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Hello, world!");

    // Get the first two command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        error!("Usage: {} <card_id> <pin>", args[0]);
        return;
    }
    let card_id = &args[1];
    let pin = &args[2];

    // Create a Reqwest client with TLS Keylog enabled
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
    );
    let mut tls_client = rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::aws_lc_rs::default_provider()))
        .with_safe_default_protocol_versions().expect("Failed to set protocol versions")
        .with_root_certificates(root_store)
        .with_no_client_auth();
    tls_client.key_log = std::sync::Arc::new(rustls::KeyLogFile::new());
    let http = reqwest::ClientBuilder::new()
        .use_preconfigured_tls(tls_client)
        .cookie_store(true)
        .build()
        .expect("Failed to create Reqwest client");

    // Get the Instance webconfig
    let webconfig = match get_config_with_client(&http, "https://bar.telecomnancy.net").await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error retrieving webconfig: {}", e);
            return;
        }
    };

    debug!("WebConfig: {:?}", webconfig);

    // Connect to OpenBar API
    let mut client = OpenBarClient::with_client(&webconfig.api, http);
    client.set_local_token(&webconfig.local_token);
    let auth_api = client.as_auth();

    // Login
    let auth_req = ConnectCardRequest::new(card_id.clone(), pin.clone());
    match auth_api.connect_card(Some(auth_req)).await {
        Ok(resp) => info!("Logged in successfully"),
        Err(e) => {
            error!("Error during login: {:?}", e);
            return;
        }
    }

    // Get all products
    let categories_api = client.as_categories();
    match categories_api.get_categories(None).await {
        Ok(categories) => {
            info!("Got {} categories:", categories.len());
            // - For each category, get items
            let items_api = client.as_items();
            for category in categories {
                let category_id = category.id;
                match items_api.get_category_items(&category_id.to_string(), Some(0), Some(100), None).await {
                    Ok(items) => {
                        info!("Items in category {}:", category.name);
                        for item in items.items {
                            let status : &str;
                            if item.state == ItemState::ItemNotBuyable {
                                status = "⛔";
                            } else if item.amount_left > 0 {
                                status = "✅";
                            } else {
                                status = "❌";
                            }
                            info!(" - {} {} (ID: {})", status, item.name, item.id);
                        }
                    }
                    Err(e) => error!("Error retrieving items for category {}: {:?}", category.name, e),
                }
            }
        },
        Err(e) => error!("Error retrieving categories: {:?}", e),
    }

    // Logout
    match auth_api.logout().await {
        Ok(_) => info!("Logged out successfully"),
        Err(e) => error!("Error during logout: {}", e),
    }
}
