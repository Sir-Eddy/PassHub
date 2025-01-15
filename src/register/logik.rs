use super::{api, view};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use directories::ProjectDirs;
use regex::Regex;
use std::{fs, io::Write};
use zeroize::Zeroize;

pub fn register(backend_url: &String) -> (String, String) {
    loop {
        // Prompt for username and password
        let (email, mut cleartext_password) = view::draw_register_screen();

        // Hash the password
        match hash_argon_2(&cleartext_password, &email) {
            Ok(password_hash) => {
                cleartext_password.zeroize(); // Clear plaintext password from memory

                // Proceed with backend login
                match api::login_backend(&backend_url, &email, &password_hash) {
                    Ok(token) => {
                        save_email_to_storage(&email); // Save email
                        return (token, password_hash); // Return JWT token
                    }
                    Err(status) => {
                        match status {
                            400 => view::error_bad_request(), // Bad request
                            409 => view::error_user_exists(), // Already exists
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

fn hash_argon_2(password: &str, email: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::encode_b64(email.as_bytes())?;
    let params = Params::new(65536, 3, 1, None)?; // 64 MiB, 3 iterations, 1 lane/thread
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    // (Optional) Verify the hash to ensure correctness
    let parsed_hash = PasswordHash::new(&password_hash)?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

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

pub fn validate_password(password: &str) -> bool {
    // Condition: Password must be at least 10 characters long
    if password.len() < 10 {
        return false;
    }

    // Regex criteria for password validation
    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password); // Uppercase letters
    let has_lowercase = Regex::new(r"[a-z]").unwrap().is_match(password); // Lowercase letters
    let has_number = Regex::new(r"\d").unwrap().is_match(password); // Numbers
    let has_special_char = Regex::new(r"[!@#$%^&*(),.?\:{}|<>]")
        .unwrap()
        .is_match(password); // Special characters

    // All criteria must be met
    has_uppercase && has_lowercase && has_number && has_special_char
}
