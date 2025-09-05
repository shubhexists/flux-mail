use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub version: u8,
    pub otp: String,
    pub mail: String,
}

pub async fn send_webhook(webhook_url: &str, payload: &Payload) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client.post(webhook_url).json(payload).send().await?;

    if res.status().is_success() {
        println!("Webhook sent to {} (status {})", webhook_url, res.status());
        Ok(())
    } else {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("Webhook failed with status {}: {}", status, body);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error: {}", status),
        )))
    }
}
