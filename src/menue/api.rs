use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::blocking::Client;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn fetch(
    backend_url: &String,
    jwt_token: &String,
    master_key: &str,
) -> Result<(u16, Option<Value>), Box<dyn std::error::Error>> {
    // Create HTTP client
    let client = Client::new();
    let request_url = format!("{}/api/v1/sync/fetch", backend_url);

    // Fetch API data
    let response = client
        .get(&request_url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()?;

    let status_code = response.status().as_u16();

    match status_code {
        200 => {
            // Check if a JSON response is present
            let json_response: Option<Value> = response.json().ok();

            // If the JSON response is empty, return only the status code
            if json_response.is_none() || json_response == Some(Value::Null) {
                return Ok((status_code, None));
            }

            // Extract Base64 string from the JSON response
            let json_response = json_response.unwrap();
            let base64_data = json_response["encrypted_data"].as_str().unwrap_or("");

            // Base64 decoding
            let decoded_data = STANDARD.decode(base64_data)?;

            // Check if minimum length is met for AES-GCM decryption
            if decoded_data.len() < 12 {
                return Ok((status_code, None));
            }

            // Convert user key to bytes
            let key = derive_key_from_hash(master_key);
            let nonce = &decoded_data[..12]; // Use the first 12 bytes as nonce
            let ciphertext = &decoded_data[12..]; // Rest as ciphertext

            // Decrypt data
            let decrypted_data = decrypt_aes256_gcm(&key, nonce, ciphertext);

            // Convert to JSON
            let json_data: Value = serde_json::from_slice(&decrypted_data)?;

            Ok((status_code, Some(json_data)))
        }
        401 | 500 => Ok((status_code, None)),
        _ => Ok((status_code, None)),
    }
}

fn decrypt_aes256_gcm(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    // Initialize AES-GCM
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    // Decrypt
    cipher.decrypt(nonce, ciphertext).unwrap()
}

fn derive_key_from_hash(password_hash: &str) -> [u8; 32] {
    // Compute SHA256 from the hash
    let mut hasher = Sha256::default();
    hasher.update(password_hash.as_bytes());
    let result = hasher.finalize();

    // Return 32-byte key
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}

pub fn update(
    backend_url: &String,
    jwt_token: &String,
    master_key: &str,
    json_data: &Value,
) -> Result<u16, Box<dyn std::error::Error>> {
    // Create HTTP client
    let client = Client::new();
    let request_url = format!("{}/api/v1/sync/update", backend_url);

    // Convert user key to bytes
    let key = derive_key_from_hash(master_key);

    // Initialize encryption
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(&key));

    // Generate nonce (12 random bytes)
    let nonce = generate_random_nonce();

    // Serialize JSON data to a string
    let json_string = serde_json::to_string(&json_data)?;

    // Encrypt JSON data
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), json_string.as_bytes())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    // Combine nonce and ciphertext
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend(ciphertext);

    // Create Base64-encoded data
    let base64_data = STANDARD.encode(&encrypted_data);

    // Embed Base64 data in JSON structure
    let json_request = serde_json::json!({ "encrypted_data": base64_data });

    // Send request
    let response = client
        .post(&request_url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&json_request)
        .send()?;

    // Return HTTP status code
    Ok(response.status().as_u16())
}

fn generate_random_nonce() -> [u8; 12] {
    use rand::Rng;

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill(&mut nonce);
    nonce
}

pub fn logout(backend_url: &String, jwt_token: &String) -> Result<u16, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_url = format!("{}/api/v1/account/logout", backend_url);

    let response = client
        .get(&request_url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()?;

    let status_code = response.status().as_u16();

    Ok(status_code)
}
