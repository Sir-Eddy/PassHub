use reqwest::blocking::Client;
use serde_json::json;

pub fn login_backend(base_url: &str, email: &str, password_hash: &str) -> Result<String, u16> {
    let client = Client::new();
    let url = format!("{}/api/v1/auth/register", base_url); // Kombiniere Basis-URL mit API-Endpunkt

    let payload = json!({
        "email": email,
        "password_hash": password_hash
    });

    let response = client.post(&url).json(&payload).send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Erfolgreich: JSON-Token extrahieren
                let body: serde_json::Value = res.json().unwrap_or_else(|_| json!({}));
                if let Some(token) = body.get("token").and_then(|t| t.as_str()) {
                    return Ok(token.to_string());
                }
                Err(500) // Falls kein Token vorhanden ist, interner Serverfehler
            } else {
                // Gib den HTTP-Statuscode bei Fehlern zurück
                Err(res.status().as_u16())
            }
        }
        Err(_) => Err(500), // Netzwerkfehler oder anderes Problem
    }
}
