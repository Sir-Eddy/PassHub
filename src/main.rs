mod view;
mod url_check {
    pub mod view;
    pub mod api;
    pub mod logik;
}
mod login {
    pub mod view;
    pub mod api;
    pub mod logik;
}
mod register {
    pub mod view;
    pub mod api;
    pub mod logik;
}
mod menue {
    pub mod view;
    pub mod api;
    pub mod logik;
}

use log::debug;
use env_logger;

use std::fs::File;
use std::io::Write;

//Main Funktion
fn main() {

    //Logging initialisieren
    env_logger::init();

    //Willkommensnachricht anzeigen
    let first_time = view::draw_welcome_screen();


    //Loop - JWT Token und Passwort Hash abfragen, danach Menü anzeigen
    loop {
    //Abruf der BackendURL
    let backend_url: String = url_check::logik::get_backend_url();
    debug!("Backend URL: {}", backend_url);


    let token: String;
    let password_hash: String;
    //Login
    match first_time {
        Some('r') => {
            // Registrierung aufrufen
            (token, password_hash) = register::logik::register(&backend_url);
        }
        _ => {
            // Login aufrufen
            (token, password_hash) = login::logik::login(&backend_url);
        }
    }

    //TODO - ONLY FOR TESTING - Token in Datei schreiben
    let mut file = File::create("token.txt").unwrap();
    file.write_all(token.as_bytes()).unwrap();

    /* 
    //TODO - ONLY FOR TESTING - REMOVE LATER
    let json_data = serde_json::json!([
        {
            "id": "25f556d4-a215-45bb-a13a-9bccee4eb005",
            "name": "10.74.0.1",
            "notes": null,
            "login": {
                "uris": [
                    {
                        "uri": "https://10.74.0.1:444/index.php?login"
                    }
                ],
                "username": null,
                "password": "password123",
                "totp": null
            }
        },
        {
            "id": "381cb892-35f5-4c9b-88c9-46bcc825da64",
            "name": "academy.hackthebox.com",
            "notes": "hello\ntest\nhello",
            "login": {
                "uris": [
                    {
                        "uri": "https://academy.hackthebox.com/login"
                    }
                ],
                "username": "test@protonmail.com",
                "password": "password!123",
                "totp": "otpauth://totp/HTB+Academy%3Aalexstephan005%40protonmail.com?secret=ABCDEFGHIJKLMNOP"
            }
        }
    ]);
    match menue::api::update(&backend_url, &token, &password_hash, &json_data) {
        Ok(status_code) => println!("Daten erfolgreich ans Backend gesendet. Statuscode: {}", status_code),
        Err(e) => println!("Fehler beim Senden der Daten ans Backend: {:?}", e),
    }

    //TODO - ONLY FOR TESTING - REMOVE LATER
    let file_name = "output.json";
    match std::fs::write(file_name, json_data.to_string()) {
        Ok(_) => println!("Daten erfolgreich in {} gespeichert.", file_name),
        Err(e) => println!("Fehler beim Schreiben der Datei: {:?}", e),
    }
    */

    menue::logik::main_menue(&backend_url, &token, &password_hash);
    }
    
}
