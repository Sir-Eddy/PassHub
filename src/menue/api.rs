use reqwest::blocking::Client;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use serde_json::Value;
use sha2::{Digest, Sha256};


pub fn fetch(backend_url: &String, jwt_token: &String, user_password_hash: &String,) -> Result<(u16, Option<Value>), Box<dyn std::error::Error>> {
    // HTTP-Client erstellen
    let client = Client::new();
    let request_url = format!("{}/api/v1/sync/fetch", backend_url);

    // API-Daten abrufen
    let response = client
        .get(&request_url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()?;

    let status_code = response.status().as_u16();

    match status_code {
        200 => {

            // Prüfen, ob eine JSON-Antwort vorhanden ist
            let json_response: Option<Value> = response.json().ok();

            // Wenn die JSON-Antwort leer ist, nur den Statuscode zurückgeben
            if json_response.is_none() || json_response == Some(Value::Null) {
                return Ok((status_code, None));
            }

            // Base64-String aus der JSON-Antwort extrahieren
            let json_response = json_response.unwrap();
            let base64_data = json_response["data"].as_str().unwrap_or("");

            // Base64-Dekodierung
            let decoded_data = STANDARD.decode(&base64_data)?;

            // Check if minimum length is met for AES-GCM decryption
            if decoded_data.len() < 12 {
                return Ok((status_code, None));
            }

            // Benutzer-Schlüssel in Bytes umwandeln
            let key = derive_key_from_hash(&user_password_hash);
            let nonce = &decoded_data[..12]; // Die ersten 12 Bytes als Nonce verwenden
            let ciphertext = &decoded_data[12..]; // Rest als Ciphertext

            // Daten entschlüsseln
            let decrypted_data = decrypt_aes256_gcm(&key, &nonce, &ciphertext);

            // In JSON umwandeln
            let json_data: Value = serde_json::from_slice(&decrypted_data)?;

            Ok((status_code, Some(json_data)))
        }
        401 | 500 => Ok((status_code, None)),
        _ => return Ok((status_code, None))
    }
}



fn decrypt_aes256_gcm(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    // AES-GCM initialisieren
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    // Entschlüsseln
    cipher.decrypt(nonce, ciphertext).unwrap()
}

fn derive_key_from_hash(password_hash: &str) -> [u8; 32] {
    // SHA256 aus dem Hash berechnen
    let mut hasher = Sha256::default();
    hasher.update(password_hash.as_bytes());
    let result = hasher.finalize();

    // 32-Byte-Schlüssel zurückgeben
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}



pub fn update(backend_url: &String, jwt_token: &String, user_password_hash: &String, data: &Value) -> Result<u16, Box<dyn std::error::Error>> {
    // HTTP-Client erstellen
    let client = Client::new();
    let request_url = format!("{}/api/v1/sync/update", backend_url);

    // Daten in JSON-String umwandeln
    let json_data = serde_json::to_string(data)?;

    // Benutzer-Schlüssel in Bytes umwandeln
    let key = derive_key_from_hash(&user_password_hash);

    // Initialisierung der Verschlüsselung
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(&key));

    // Nonce generieren (12 zufällige Bytes)
    let nonce = generate_random_nonce();

    // JSON-Daten verschlüsseln
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), json_data.as_bytes())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    // Nonce und Ciphertext kombinieren
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend(ciphertext);

    // Base64-kodierte Daten erstellen
    let base64_data = STANDARD.encode(&encrypted_data);

    // Base64-Daten in JSON-Struktur einbetten
    let json_request = serde_json::json!({ "data": base64_data });

    // Anfrage senden
    let response = client
        .post(&request_url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&json_request)
        .send()?;

    // HTTP-Statuscode zurückgeben
    Ok(response.status().as_u16())
}

fn generate_random_nonce() -> [u8; 12] {
    use rand::Rng;

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill(&mut nonce);
    nonce
}
