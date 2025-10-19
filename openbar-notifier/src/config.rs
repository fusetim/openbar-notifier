use std::path::PathBuf;

/// Global configuration for OpenBar Notifier
#[derive(Debug, Clone, Default)]
pub struct GlobalConfig {
    /// OpenBar connection configuration
    pub openbar: OpenBarConfig,
    /// Notification configuration
    pub notify: NotifyConfig,
    /// Notification targets (list of webhook URLs)
    pub targets: Vec<String>,
    /// Persistent store file path
    pub store_file: PathBuf,
}

/// OpenBar connection configuration
#[derive(Debug, Clone, Default)]
pub struct OpenBarConfig {
    /// OpenBar instance URL
    pub instance_url: String,
    /// Card ID for login
    pub card_id: String,
    /// PIN for the card
    pub pin: String,
}

/// Notification configuration
#[derive(Debug, Clone, Default)]
pub struct NotifyConfig {
    /// Notify when a new item is added
    pub item_added: bool,
    /// Notify when an item becomes buyable
    pub become_buyable: bool,
    /// Notify when an item becomes unbuyable
    pub become_unbuyable: bool,
    /// Notify when an item is out of stock
    pub on_out_of_stock: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalConfigLoadError {
    MissingOpenBarInstanceUrl,
    MissingCardId,
    MissingPin,
}

impl GlobalConfig {
    /// Load configuration from environment variables
    ///
    /// Environment Variables:
    /// - STORE_PATH (optional): Path to the persistent store file (default: "./item_store.json")
    /// - OPENBAR_INSTANCE_URL (required): URL of the OpenBar instance
    /// - OPENBAR_CARD_ID (required): Card ID for login
    /// - OPENBAR_PIN (required): PIN for the card
    /// - NOTIFY_ITEM_ADDED (default: false): Notify when a new item is added
    /// - NOTIFY_BECOME_BUYABLE (default: false): Notify when an item becomes buyable
    /// - NOTIFY_BECOME_UNBUYABLE (default: false): Notify when an item becomes unbuyable
    /// - NOTIFY_ON_OUT_OF_STOCK (default: false): Notify when an item is out of stock
    /// - NOTIFICATION_TARGETS: Comma-separated list of notification target URLs
    pub fn load_env() -> Result<Self, GlobalConfigLoadError> {
        let store_file =
            std::env::var("STORE_PATH").unwrap_or_else(|_| "./item_store.json".to_string());
        let instance_url = std::env::var("OPENBAR_INSTANCE_URL")
            .map_err(|_| GlobalConfigLoadError::MissingOpenBarInstanceUrl)?;
        let card_id =
            std::env::var("OPENBAR_CARD_ID").map_err(|_| GlobalConfigLoadError::MissingCardId)?;
        let pin = std::env::var("OPENBAR_PIN").map_err(|_| GlobalConfigLoadError::MissingPin)?;

        let item_added = std::env::var("NOTIFY_ITEM_ADDED")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let become_buyable = std::env::var("NOTIFY_BECOME_BUYABLE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let become_unbuyable = std::env::var("NOTIFY_BECOME_UNBUYABLE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";
        let on_out_of_stock = std::env::var("NOTIFY_ON_OUT_OF_STOCK")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";

        let targets = std::env::var("NOTIFICATION_TARGETS")
            .unwrap_or_else(|_| "".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(GlobalConfig {
            store_file: PathBuf::from(store_file),
            openbar: OpenBarConfig {
                instance_url,
                card_id,
                pin,
            },
            notify: NotifyConfig {
                item_added,
                become_buyable,
                become_unbuyable,
                on_out_of_stock,
            },
            targets,
        })
    }
}
