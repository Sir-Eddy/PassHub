use zeroize::Zeroize;

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
    let mut first_time = view::draw_welcome_screen();

    //Loop - Query JWT token and password hash, then display the menu
    loop {
        //Get backend URL
        let backend_url: String = url_check::logik::get_backend_url();
        let token: String;
        let mut master_key: String;

        match first_time {
            Some('r') => {
                (token, master_key) = register::logik::register(&backend_url);
                first_time = Some('l'); // Set screen to login after next logout
            }
            Some('d') => {
                (token, master_key) = login::logik::login(&backend_url);
                delete::logik::delete(&backend_url, &token);
                master_key.zeroize();
                std::process::exit(0);
            }
            _ => {
                (token, master_key) = login::logik::login(&backend_url);
            }
        }
        menue::logik::main_menue(&backend_url, &token, master_key);
    }
}
