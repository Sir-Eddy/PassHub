use rand::rngs::OsRng;
use argon2::{
    password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2, Algorithm, Params, Version,
};
use ratatui::backend;
use core::hash;
use std::{io::Read, io::Write, string, fs};
use directories::ProjectDirs;
use zeroize::Zeroize;
use super::{api, view};

pub fn login(backend_url: &String) -> String {
    loop {

    // E-Mail aus dem Speicher laden
    let stored_email = get_mail_from_storage();

    // Benutzername und Passwort abfragen
    let (email, mut cleartext_password) = view::draw_login_screen(stored_email);

    // Passwort hashen
    match hash_argon_2(&cleartext_password, &email) {
        Ok(password_hash) => {
            cleartext_password.zeroize(); // Klartext-Passwort aus dem Speicher löschen

            // Weiter mit dem Backend-Login
            match api::login_backend(&backend_url, &email, &password_hash) {
                Ok(token) => {
                    save_email_to_storage(&email); // E-Mail speichern
                    return token; // JWT-Token zurückgeben
                }
                Err(status) => {
                    match status {
                        400 => view::error_bad_request(),    // Bad request
                        401 => view::error_unauthorized(),  // Unauthorized
                        404 => view::error_user_not_found(),     // Not found
                        500 => view::error_network(),  // Internal server error
                        _ => view::error_unknown(), // Unknown error
                    }
                }
            }
        }
        Err(e) => {
            cleartext_password.clear(); // Klartext-Passwort aus dem Speicher löschen
            view::error_argon2_fail();
            std::process::exit(1);
        }
        }
    }
}

fn hash_argon_2(password: &str, email: &str) -> Result<String, argon2::password_hash::Error> {

    let salt = SaltString::b64_encode(email.as_bytes())?;
    let params = Params::new(65536, 3, 1, None)?; // 64 MiB, 3 iterations, 1 lane/thread
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    // (Optional) Verify the hash to ensure correctness
    let parsed_hash = PasswordHash::new(&password_hash)?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(password_hash)
}

fn get_mail_from_storage () -> String {
    // Hol das Projektverzeichnis
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("mail.txt");

        // Falls die Datei existiert, lese den Inhalt
        if config_file.exists() {
            let mut file = fs::File::open(&config_file).expect("Fehler beim Öffnen der Datei");
            let mut mail = String::new();
            file.read_to_string(&mut mail).expect("Fehler beim Lesen der Datei");

            // Entferne Whitespaces und prüfe, ob eine URL vorhanden ist
            let mail = mail.trim();
            if mail.is_empty() {
                return String::new();
            } else {
                return mail.to_string();
            }
        }
        else {
            return String::new();
        }
    }
    else {
        return String::new();
    }
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