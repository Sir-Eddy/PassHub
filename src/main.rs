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
mod delete {
    pub mod api;
    pub mod logik;
    pub mod view;
}

//Main function
fn main() {
    //Display welcome screen
    let first_time = view::draw_welcome_screen();

    //Loop - Query JWT token and password hash, then display the menu
    loop {
        //Get backend URL
        let backend_url: String = url_check::logik::get_backend_url();
        let token: String;
        let password_hash: String;

        match first_time {
            Some('r') => {
                (token, password_hash) = register::logik::register(&backend_url);
            }
            Some('d') => {
                (token, _) = login::logik::login(&backend_url);
                delete::logik::delete(&backend_url, &token);
                std::process::exit(1);
            }
            _ => {
                (token, password_hash) = login::logik::login(&backend_url);
            }
        }
        menue::logik::main_menue(&backend_url, &token, &password_hash);
    }
}
