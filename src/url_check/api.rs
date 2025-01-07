use log::debug;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::thread::sleep;
use std::time::Duration;

pub fn check_health(base_url: &str) -> bool {
    // Erstelle die vollständige URL für die Health-Route
    let health_url = format!("{}/api/v1/health", base_url);

    // Erstelle einen HTTP-Client
    let client = Client::new();

    // Versuche bis zu 3 Mal, die Health-Route zu prüfen
    for attempt in 1..=3 {
        match client.get(&health_url).send() {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    return true; // Erfolg
                }
            }
            Err(_) => {
                debug!("Versuch {} fehlgeschlagen.", attempt);
            }
        }
        // Wartezeit vor dem nächsten Versuch (optional)
        if attempt < 3 {
            sleep(Duration::from_secs(1)); // Warte 1 Sekunde zwischen den Versuchen
        }
    }

    // Wenn alle Versuche scheitern, gib false zurück
    false
}
