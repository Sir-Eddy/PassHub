use reqwest::blocking::Client;
use serde_json::json;

pub fn login_backend(base_url: &str, email: &str, password_hash: &str) -> Result<String, u16> {
    let client = Client::new();
    let url = format!("{}/api/v1/auth/register", base_url); // Combine base URL with API endpoint

    let payload = json!({
        "email": email,
        "password_hash": password_hash
    });

    let response = client.post(&url).json(&payload).send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Success: Extract JSON token
                let body: serde_json::Value = res.json().unwrap_or_else(|_| json!({}));
                if let Some(token) = body.get("token").and_then(|t| t.as_str()) {
                    return Ok(token.to_string());
                }
                Err(500) // If no token is present, internal server error
            } else {
                // Return the HTTP status code on errors
                Err(res.status().as_u16())
            }
        }
        Err(_) => Err(500), // Network error or other problem
    }
}
