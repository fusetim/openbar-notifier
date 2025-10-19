use std::sync::Arc;

use dotenv::dotenv;
use log::{debug, error, info, warn};
use openbar_api::models::ItemState;
use openbar_notifier::config::GlobalConfig;
use openbar_notifier::event::ItemEvent;
use openbar_notifier::openbar::{OpenBarClient, webconfig::get_config_with_client};
use serde_json::json;
use std::io::Write;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Hello, world!");

    // Get the configuration from environment variables
    let config = match GlobalConfig::load_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Error loading configuration: {:?}", e);
            return;
        }
    };

    // Create a Reqwest client with TLS Keylog enabled
    let http = create_http_client();

    // Get the Instance webconfig
    let webconfig = match get_config_with_client(&http, &config.openbar.instance_url).await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error retrieving webconfig: {}", e);
            return;
        }
    };

    debug!("WebConfig: {:?}", webconfig);

    // Connect to OpenBar API
    let mut client = OpenBarClient::with_client(&webconfig.api, http.clone());
    client.set_local_token(&webconfig.local_token);

    // Login
    match client
        .login_by_card(&config.openbar.card_id, &config.openbar.pin)
        .await
    {
        Ok(_resp) => info!("Logged in successfully"),
        Err(e) => {
            error!("Error during login: {:?}", e);
            return;
        }
    }

    // Load the item store from the file
    let mut item_store = match load_item_store_from_file(&config.store_file) {
        Ok(store) => store,
        Err(e) => {
            error!("Error loading item store: {}", e);
            return;
        }
    };

    // Store the item events to process later
    let mut item_events: Vec<(Uuid, ItemEvent)> = Vec::new();

    // Get all products
    match client.get_categories().await {
        Ok(categories) => {
            info!("Got {} categories:", categories.len());
            // - For each category, get items
            for category in categories {
                let category_id = category.id;
                match client.get_category_items(&category_id.to_string()).await {
                    Ok(items) => {
                        info!("{} items in category {}:", items.len(), category.name);
                        for item in items {
                            // Check if the item is already in the store
                            if let Some(existing) = item_store.find_mut(item.id) {
                                // Compare states to determine events
                                if existing.state != item.state {
                                    match item.state {
                                        ItemState::ItemBuyable => {
                                            item_events.push((item.id, ItemEvent::BecomeBuyable))
                                        }
                                        ItemState::ItemNotBuyable => {
                                            item_events.push((item.id, ItemEvent::BecomeUnbuyable))
                                        }
                                    }
                                }
                                if existing.amount_left > 0 && item.amount_left == 0 {
                                    item_events.push((item.id, ItemEvent::OutOfStock))
                                }
                                // Update existing item
                                *existing = item.clone();
                            } else {
                                // New item, add to store
                                item_store.append(item.clone());
                                item_events.push((item.id, ItemEvent::Added));
                                info!("New item added: {} (ID: {})", item.name, item.id);
                            }
                        }
                    }
                    Err(e) => error!(
                        "Error retrieving items for category {}: {:?}",
                        category.name, e
                    ),
                }
            }
        }
        Err(e) => error!("Error retrieving categories: {:?}", e),
    }

    // Logout
    match client.logout().await {
        Ok(_) => info!("Logged out successfully"),
        Err(e) => error!("Error during logout: {}", e),
    }

    // Process item events (notifications, etc.)
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    for (item_id, event) in item_events {
        let item = match item_store.find(item_id) {
            Some(i) => i,
            None => {
                warn!(
                    "Item ID {} not found in store for event processing.",
                    item_id
                );
                continue;
            }
        };
        match event {
            ItemEvent::Added if config.notify.item_added => writer
                .write_all(format!("- {} ({}) added.\n", &item.name, item_id).as_bytes())
                .unwrap(),
            ItemEvent::BecomeBuyable if config.notify.become_buyable => writer
                .write_all(
                    format!(
                        "- {} ({}) became buyable (stock: {}).\n",
                        &item.name, item_id, item.amount_left
                    )
                    .as_bytes(),
                )
                .unwrap(),
            ItemEvent::BecomeUnbuyable if config.notify.become_unbuyable => writer
                .write_all(format!("- {} ({}) became unbuyable.\n", &item.name, item_id).as_bytes())
                .unwrap(),
            ItemEvent::OutOfStock if config.notify.on_out_of_stock => writer
                .write_all(format!("- {} ({}) is out of stock.\n", &item.name, item_id).as_bytes())
                .unwrap(),
            _ => { /* Notification for this event type is disabled */ }
        }
    }

    if !buf.is_empty() {
        // If buf > 2000 bytes, truncate and add notice
        if buf.len() > 2000 {
            buf.truncate(1800);
            let notice = b"\n... (truncated)";
            buf.extend_from_slice(notice);
        }

        for target in &config.targets {
            info!("Notifying target {}...", target);
            let json_body = json!({ "content": String::from_utf8_lossy(&buf) });
            let res = http.post(target).json(&json_body).send().await;
            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        info!("Notification sent successfully to {}", target);
                    } else {
                        error!(
                            "Failed to send notification to {}: HTTP {}",
                            target,
                            resp.status()
                        );
                        dbg!(resp.text().await.unwrap_or_default());
                    }
                }
                Err(e) => {
                    error!("Error sending notification to {}: {}", target, e);
                }
            }
        }
    } else {
        info!("No item events to notify.");
    }

    // Save the item store back to the file
    if let Err(e) = save_item_store_to_file(&item_store, &config.store_file) {
        error!("Error saving item store: {}", e);
    }
}

/// Create a Reqwest HTTP client with TLS Keylog support (easier to debug).
fn create_http_client() -> reqwest::Client {
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let mut tls_client = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("Failed to set protocol versions")
    .with_root_certificates(root_store)
    .with_no_client_auth();
    tls_client.key_log = std::sync::Arc::new(rustls::KeyLogFile::new());
    reqwest::ClientBuilder::new()
        .use_preconfigured_tls(tls_client)
        .cookie_store(true)
        .build()
        .expect("Failed to create Reqwest client")
}

/// Load from file, the item store
fn load_item_store_from_file(
    path: &std::path::Path,
) -> Result<openbar_notifier::store::ItemStore, Box<dyn std::error::Error>> {
    // Check if the file exists
    if !path.exists() {
        // If not, return an empty store
        warn!(
            "Store file does not exist at {:?}, starting with an empty store.",
            path
        );
        return Ok(openbar_notifier::store::ItemStore::new());
    }
    let data = std::fs::read_to_string(path)?;
    let store: openbar_notifier::store::ItemStore = serde_json::from_str(&data)?;
    Ok(store)
}

/// Save the item store to a file
fn save_item_store_to_file(
    store: &openbar_notifier::store::ItemStore,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(store)?;
    std::fs::write(path, data)?;
    Ok(())
}
