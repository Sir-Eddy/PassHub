use serde_json::Error;
use serde_json::Value;
use super::{api, view};

pub fn main_menue(backend_url: &String, token: &String, password_hash: &String) {
    // Get the passwords from the backend
    let json_data_result = api::fetch(&backend_url, &token, &password_hash);

    match json_data_result {
        Ok((_status_code, Some(json_data))) => {
            let _ = view::display_data(json_data);
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
        //TODO - ALWAYS RETURNING UNKNOWN ERROR - WHY?
        Err(_e) => {
            view::unknown_error();
            return;
        }
    }
}

pub fn add_password(){
    todo!();
}

pub fn delete_user(){
    todo!();
}

pub fn get_uris(json_data: Value)->Result<Vec<std::string::String>, Error>{
    //array für die Speicherung der uris anlegen
    let mut uris = Vec::new();


    //get all uris from all objects
    if let Value::Array(entries) = json_data {
        for entry in entries {
            if let Some(login) = entry.get("login") {
                if let Some(uris_array) = login.get("uris") {
                    if let Value::Array(uris_list) = uris_array {
                        for uri_object in uris_list {
                            if let Some(uri) = uri_object.get("uri") {
                                if let Value::String(uri_str) = uri {
                                    uris.push(uri_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return Ok(uris);

}