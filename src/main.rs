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
mod password_display {
    pub mod view;
    pub mod api;
    pub mod logik;
}

use log::debug;
use env_logger;

//Main Funktion
fn main() {

    //Logging initialisieren
    env_logger::init();

    //Willkommensnachricht anzeigen
    let first_time = view::draw_welcome_screen();


    //Abruf der BackendURL
    let backend_url: String = url_check::logik::get_backend_url();
    debug!("Backend URL: {}", backend_url);

    //Login
    match first_time {
        Some('r') => {
            // Registrierung aufrufen
            let token = register::logik::register(backend_url);
        }
        _ => {
            // Login aufrufen
            let token = login::logik::login(backend_url);
            
        }
    }

    //Passwörter anzeigen
    password_display::logik::display_passwords(backend_url, token);
    
}
