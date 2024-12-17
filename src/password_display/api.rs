use ratatui::backend;
use reqwest::blocking::Client;
use reqwest::{Response, StatusCode};
use log::debug;
use serde_json::json;


fn synch(backend_url :String, token: String)->Result<String, String>{
    todo!();

        /*let vault_url = format!("{}/api/v1/sync/fetch", backend_url);
        
        let client = Client::new();
        let response = client.get(&vault_url).bearer_auth(token).send()

        match response.status() {
            StatusCode::OK => response.text().map_err(|e| format!("Failed to read response body: {}", e)), //Nochmal überarbeiten
            StatusCode::UNAUTHORIZED => {
                debug!("Error: JWT Token is invalid!");
                Err("Unauthorized (401)".to_string())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                debug!("Database Error or JWT Token Extraction Error!");
                Err("Internal Server Error (500)".to_string())
            }
            _ => {
                debug!("Unexpected response status: {}", response.status());
                Err(format!("Unexpected status: {}", response.status()))
            }
        }*/
}