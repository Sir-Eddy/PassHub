use log::debug;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::thread::sleep;
use std::time::Duration;

pub fn check_health(base_url: &str) -> bool {
    let health_url = format!("{}/api/v1/health", base_url);
    let client = Client::new();

    // Try to send a GET request to the health URL 3 times
    for attempt in 1..=3 {
        match client.get(&health_url).send() {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    return true; // Erfolg
                }
            }
            Err(_) => {
                debug!("{} try failed", attempt);
            }
        }
        if attempt < 3 {
            sleep(Duration::from_secs(1)); // Wait for 1 second before next attempt
        }
    }

    // If all attempts fail, return false
    false
}
