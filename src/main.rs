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


use log::{debug, info, warn, error};
use env_logger;

//Main Funktion
fn main() {

    //Logging initialisieren
    env_logger::init();

    //Willkommensnachricht anzeigen
    view::draw_welcome_screen();


    //Abruf der BackendURL
    let backend_url: String = url_check::logik::get_backend_url();
    debug!("Backend URL: {}", backend_url);

    //Login
    //let login_result: bool = login::logik::login(backend_url);
    
}
