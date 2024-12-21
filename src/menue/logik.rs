use super::{api, view};

pub fn main_menue(backend_url: &String, token: &String, password_hash: &String) {
    // Get the passwords from the backend
    let json_data_result = api::fetch(backend_url, token, password_hash);
    match json_data_result {
        Ok(json_data) => {
            // Überprüfen, ob es einen Fehlercode im JSON gibt
            if let Some(error_code) = json_data.get("error_code") {
                match error_code.as_i64() {
                    Some(401) => view::jwt_invalid(), // Fehlercode 401
                    Some(404) => view::internal_server_error(),   // Fehlercode 404
                    _ => view::unknown_error(),       // Standardfehlerbehandlung
                }
            } else {
                // Übergabe an die View oder weitere Verarbeitung
                view::display_data(&json_data);
            }
        }
        Err(e) => {
            // Fehler beim Abrufen oder Verarbeiten der Daten
            return;
        }
    }

}