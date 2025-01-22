use super::{api, view};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use directories::ProjectDirs;
use regex::Regex;
use lazy_static::lazy_static;
use std::{fs, io::Write};
use zeroize::Zeroize;

pub fn register(backend_url: &str) -> (String, String) {
    loop {
        // Prompt for username and password
        let (email, mut cleartext_password) = view::draw_register_screen();

        // Hash the password
        match hash_argon_2_master_key(&cleartext_password, &email) {
            Ok(mut master_key) => {
                let master_password_hash =
                    hash_argon_2_master_password_hash(&master_key, &cleartext_password);
                cleartext_password.zeroize(); // Clear plaintext password from memory

                // Proceed with backend login
                match api::login_backend(
                    backend_url,
                    &email,
                    &master_password_hash
                        .expect("Register: Error occurred extracting master password"),
                ) {
                    Ok(token) => {
                        save_email_to_storage(&email); // Save email
                        return (token, master_key); // Return JWT token
                    }
                    Err(status) => {
                        match status {
                            400 => view::error_bad_request(), // Bad request
                            409 => {
                                master_key.zeroize();
                                view::error_user_exists();
                            } // Already exists
                            500 => view::error_network(),     // Internal server error
                            _ => view::error_unknown(),       // Unknown error
                        }
                    }
                }
            }
            Err(_e) => {
                cleartext_password.clear(); // Clear plaintext password from memory
                view::error_argon2_fail();
                std::process::exit(1);
            }
        }
    }
}

fn hash_argon_2_master_key(
    password: &str,
    email: &str,
) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::encode_b64(email.as_bytes())?;
    let params = Params::new(65536, 3, 4, None)?; // 64 MiB, 3 iterations, 4 lane/thread
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    // Verify the hash to ensure correctness
    let parsed_hash = PasswordHash::new(&password_hash)?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(password_hash)
}

fn hash_argon_2_master_password_hash(
    master_key: &str,
    master_password: &str,
) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::encode_b64(master_password.as_bytes())?;
    let params = Params::new(65536, 3, 4, None)?; // 64 MiB, 3 iterations, 4 lane/thread
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let password_hash = argon2
        .hash_password(master_key.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn save_email_to_storage(email: &str) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("mail.txt");

        // Write the email to the file
        let mut file = fs::File::create(&config_file).expect("Error creating file");
        file.write_all(email.as_bytes())
            .expect("Error writing to file");
    }
}

lazy_static! {
    static ref HAS_UPPERCASE: Regex = Regex::new(r"[A-Z]").expect("Regex-Kompilierungsfehler");
    static ref HAS_LOWERCASE: Regex = Regex::new(r"[a-z]").expect("Regex-Kompilierungsfehler");
    static ref HAS_NUMBER: Regex = Regex::new(r"\d").expect("Regex-Kompilierungsfehler");
    static ref HAS_SPECIAL_CHAR: Regex = Regex::new(r"[!@#$%^&*(),.?\:{}|<>]").expect("Regex-Kompilierungsfehler");
}

pub fn validate_password(password: &str) -> bool {
    // Bedingung: Das Passwort muss mindestens 10 Zeichen lang sein
    if password.len() < 10 {
        return false;
    }

    // Überprüfe die Regex-Kriterien für das Passwort
    let has_uppercase = HAS_UPPERCASE.is_match(password); // Großbuchstaben
    let has_lowercase = HAS_LOWERCASE.is_match(password); // Kleinbuchstaben
    let has_number = HAS_NUMBER.is_match(password);       // Zahlen
    let has_special_char = HAS_SPECIAL_CHAR.is_match(password); // Sonderzeichen

    // Alle Kriterien müssen erfüllt sein
    has_uppercase && has_lowercase && has_number && has_special_char
}
