use argon2::{
    password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2, Algorithm, Params, Version,
};
use std::{io::Write, fs};
use directories::ProjectDirs;
use regex::Regex;
use zeroize::Zeroize;
use super::{api, view};

pub fn register(backend_url: &String) -> (String, String) {
    loop {

    // Benutzername und Passwort abfragen
    let (email, mut cleartext_password) = view::draw_register_screen();

    // Passwort hashen
    match hash_argon_2(&cleartext_password, &email) {
        Ok(password_hash) => {
            cleartext_password.zeroize(); // Klartext-Passwort aus dem Speicher löschen

            // Weiter mit dem Backend-Login
            match api::login_backend(&backend_url, &email, &password_hash) {
                Ok(token) => {
                    save_email_to_storage(&email); // E-Mail speichern
                    return (token, password_hash); // JWT-Token zurückgeben
                }
                Err(status) => {
                    match status {
                        400 => view::error_bad_request(),    // Bad request
                        409 => view::error_user_exists(),     // Allready exists
                        500 => view::error_network(),  // Internal server error
                        _ => view::error_unknown(), // Unknown error
                    }
                }
            }
        }
        Err(_e) => {
            cleartext_password.clear(); // Klartext-Passwort aus dem Speicher löschen
            view::error_argon2_fail();
            std::process::exit(1);
        }
        }
    }
}

fn hash_argon_2(password: &str, email: &str) -> Result<String, argon2::password_hash::Error> {

    let salt = SaltString::encode_b64(email.as_bytes())?;
    let params = Params::new(65536, 3, 1, None)?; // 64 MiB, 3 iterations, 1 lane/thread
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    // (Optional) Verify the hash to ensure correctness
    let parsed_hash = PasswordHash::new(&password_hash)?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(password_hash)
}

fn save_email_to_storage (email: &str) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("mail.txt");

        // Schreibe die URL in die Datei
        let mut file = fs::File::create(&config_file).expect("Fehler beim Erstellen der Datei");
        file.write_all(email.as_bytes()).expect("Fehler beim Schreiben in die Datei");
    }
}

pub fn validate_password(password: &str) -> bool {
    // Bedingung: Passwort muss mindestens 10 Zeichen lang sein
    if password.len() < 10 {
        return false;
    }

    // Regex-Kriterien für die Passwortprüfung
    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password); // Großbuchstaben
    let has_lowercase = Regex::new(r"[a-z]").unwrap().is_match(password); // Kleinbuchstaben
    let has_number = Regex::new(r"\d").unwrap().is_match(password);       // Zahlen
    let has_special_char = Regex::new(r"[!@#$%^&*(),.?\:{}|<>]").unwrap().is_match(password); // Sonderzeichen

    // Alle Kriterien müssen erfüllt sein
    has_uppercase && has_lowercase && has_number && has_special_char
}