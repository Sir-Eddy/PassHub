use super::{api, view};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use directories::ProjectDirs;
use std::{fs, io::Read, io::Write};
use zeroize::Zeroize;

pub fn login(backend_url: &str) -> (String, String) {
    loop {
        // Load email from storage
        let stored_email = get_mail_from_storage();

        // Prompt for username and password
        let (email, mut cleartext_password) = view::draw_login_screen(stored_email);

        // Hash the password
        match hash_argon_2_master_key(&cleartext_password, &email) {
            Ok(mut master_key) => {
                let master_password_hash =
                    hash_argon_2_master_password_hash(&master_key, &cleartext_password);
                cleartext_password.zeroize(); // Clear plaintext password from memory

                // Proceed with backend login
                match api::login_backend(backend_url, &email, &master_password_hash.unwrap()) {
                    Ok(token) => {
                        save_email_to_storage(&email); // Save email
                        return (token, master_key); // Return JWT token
                    }
                    Err(status) => {
                        match status {
                            400 => view::error_bad_request(),    // Bad request
                            401 => view::error_unauthorized(),   // Unauthorized
                            404 => {
                                    master_key.zeroize();
                                    view::error_user_not_found();}, // Not found
                            500 => view::error_network(),        // Internal server error
                            _ => view::error_unknown(),          // Unknown error
                        }
                    }
                }
            }
            Err(_e) => {
                cleartext_password.zeroize();
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

fn get_mail_from_storage() -> String {
    // Get the project directory
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("mail.txt");

        // If the file exists, read its content
        if config_file.exists() {
            let mut file = fs::File::open(&config_file).expect("Error opening the file");
            let mut mail = String::new();
            file.read_to_string(&mut mail)
                .expect("Error reading the file");

            // Remove whitespaces and check if an email is present
            let mail = mail.trim();
            if mail.is_empty() {
                String::new()
            } else {
                mail.to_string()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn save_email_to_storage(email: &str) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("mail.txt");

        // Write the email to the file
        let mut file = fs::File::create(&config_file).expect("Error creating the file");
        file.write_all(email.as_bytes())
            .expect("Error writing to the file");
    }
}
