use std::error;

use serde_json::Error;
use serde_json::Value;
use serde_json::error::Category;
use serde::{Serialize, Deserialize};
use super::{api, view};
use log::debug;

pub fn main_menue(backend_url: &String, token: &String, password_hash: &String) {
    // Get the passwords from the backend
    let json_data_result = api::fetch(&backend_url, &token, &password_hash);

    match json_data_result {
        Ok((200, Some(json_data))) => {
            let _ = view::display_data(json_data);
        }
        Ok((401, _)) => {
            view::invalid_token();
            return;
        }
        Ok((500, _)) => {
            view::database_error();
            return;
        }
        Ok((_status_code, _)) => {
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

pub fn get_uris(json_entries:Vec<Entry>)->Result<Vec<String>, Error>{
    //array für die Speicherung der uris anlegen
    let mut uris = Vec::new();
    for element in json_entries{
        for uri in element.login.uris {
            uris.push(uri.uri);
        }
    }
    return Ok(uris);

}

pub fn deserialize_json(json_data: Value)->Result<Vec<Entry>, Error>{
    todo!("Error handling noch fertig machen");
    let entries  = serde_json::from_value(json_data);
    match entries {
        Ok(entry_list)=> Ok(entry_list),
        Err(e) => {
        match e.classify() {
            Category::Io => {debug!("Failed to read or write bytes on an I/O stream");
            return Err(e);},
            Category::Syntax => {debug!("Input is not syntactically valid JSON");
            return Err(e)},
            Category::Data => {debug!("Input data is semantically incorrect");
            return Err(e);},
            Category::Eof => {debug!("Unexpected end of the input data");
            return Err(e)},
        }
    }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Uri {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    uris: Vec<Uri>,
    username: Option<String>,
    password: String,
    totp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    id: String,
    name: String,
    notes: Option<String>,
    login: Login,
}