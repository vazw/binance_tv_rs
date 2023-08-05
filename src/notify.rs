use std::collections::HashMap;
use std::env;

use reqwest;
use tracing::info;
pub async fn notify_send(message: String) {
    let url = "https://notify-api.line.me/api/notify";
    let token = match env::var("LINE_TOKEN") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }; // Replace with your LINE Notify token
    let mut messages = HashMap::new();
    messages.insert("message", message);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .form(&messages)
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        info!("Notification sent successfully!");
    } else {
        info!("Failed to send notification: {}", res.status());
    };
}
