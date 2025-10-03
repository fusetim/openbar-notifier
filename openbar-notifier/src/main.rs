use openbar_notifier::openbar::{webconfig::get_config, OpenBarClient};
use openbar_api::{apis::auth_api::AuthApi, models::ConnectCardRequest};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Get the Instance webconfig
    let webconfig = match get_config("https://bar.telecomnancy.net").await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error retrieving webconfig: {}", e);
            return;
        }
    };

    println!("WebConfig: {:?}", webconfig);

    // Connect to OpenBar API
    let mut client = OpenBarClient::new(&webconfig.api);
    client.set_local_token(&webconfig.local_token);
    let auth_api = client.as_auth();

    // Login
    let auth_req = ConnectCardRequest::new("CARD_ID".to_string(), "1234".to_string());
    match auth_api.connect_card(Some(auth_req)).await {
        Ok(resp) => println!("Logged in successfully: {:?}", resp),
        Err(e) => {
            eprintln!("Error during login: {:?}", e);
            return;
        }
    }

    // Logout
    match auth_api.logout().await {
        Ok(_) => println!("Logged out successfully"),
        Err(e) => eprintln!("Error during logout: {}", e),
    }
}
