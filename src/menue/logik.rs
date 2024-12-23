use super::{api, view};

pub fn main_menue(backend_url: &String, token: &String, password_hash: &String) {
    // Get the passwords from the backend
    let json_data_result = api::fetch(&backend_url, &token, &password_hash);

    match json_data_result {
        Ok((_status_code, Some(json_data))) => {
            view::display_data(&json_data);
        }
        Ok((401, None)) => {
            view::invalid_token();
            return;
        }
        Ok((500, None)) => {
            view::database_error();
            return;
        }
        Ok((_status_code, None)) => {
            view::unknown_error();
            return;
        }
        Err(_e) => {
            view::unknown_error();
            return;
        }
    }
}