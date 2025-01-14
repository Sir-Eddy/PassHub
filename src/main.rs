mod view;
mod url_check {
    pub mod api;
    pub mod logik;
    pub mod view;
}
mod login {
    pub mod api;
    pub mod logik;
    pub mod view;
}
mod register {
    pub mod api;
    pub mod logik;
    pub mod view;
}
mod menue {
    pub mod api;
    pub mod logik;
    pub mod view;
}

use env_logger;
use log::debug;


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
        menue::logik::main_menue(&backend_url, &token, &password_hash);
    }
}
