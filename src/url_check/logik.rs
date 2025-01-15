use super::api;
use super::view;

use directories::ProjectDirs;
use std::fs;
use std::io::Read;
use std::io::Write;
use url::Url;

pub fn get_backend_url() -> String {
    let mut backend_url = is_url_in_storage();

    //Check availability of stored URL
    if let Some(ref url) = backend_url {
        if !api::check_health(url) {
            view::error_url_unreachable(&Some(url.clone()));
            backend_url = None; //Set to None to force new input
        }
    }

    loop {
        if backend_url.is_none() {
            create_file(); //Create file if not present
            loop {
                let temp_url: String = view::ask_for_url();
                if Url::parse(&temp_url).is_ok() {
                    save_backend_url(&temp_url);
                    backend_url = Some(temp_url);
                    break;
                } else {
                    view::error_url_unavailable();
                }
            }
        }

        //Check if entered URL is reachable
        if let Some(ref url) = backend_url {
            if api::check_health(url) {
                return url.clone(); //URL is reachable
            } else {
                view::error_url_unreachable(&Some(url.clone()));
                backend_url = None; //Set to None to force new input
            }
        }
    }
}

//Returns the URL if it is stored in the file
fn is_url_in_storage() -> Option<String> {
    //Get the project directory
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        //If the file exists, read the URL
        if config_file.exists() {
            let mut file = fs::File::open(&config_file).expect("Error opening the file");
            let mut url = String::new();
            file.read_to_string(&mut url)
                .expect("Error reading the file");

            //Remove leading and trailing whitespaces
            let trimmed_url = url.trim();
            if !trimmed_url.is_empty() {
                return Some(trimmed_url.to_string());
            }
        }
    }
    None
}

//Creates the file if it does not exist
fn create_file() {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        fs::create_dir_all(config_dir).expect("Error creating directory");
        fs::File::create(&config_file).expect("Error creating file");
    }
}

//Save the URL to the file if it is valid
fn save_backend_url(url: &str) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        // Write the URL to the file
        let mut file = fs::File::create(&config_file).expect("Error creating the file");
        file.write_all(url.as_bytes())
            .expect("Error writing the file");
    }
}
