use super::{api, view};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::error::Category;
use serde_json::Error;
use serde_json::Value;

pub fn main_menue(backend_url: &String, token: &String, password_hash: &str) {
    loop {
        // Get the passwords from the backend
        let json_data_result = api::fetch(backend_url, token, password_hash);

        match json_data_result {
            Ok((200, Some(json_data))) => {
                let entry_return = view::display_data(&json_data).unwrap();
                let entries = entry_return.0;

                let json_string = serialize_json(&entries).unwrap();
                let json_string = json_string.as_str();
                let json_value: Value = match serde_json::from_str(json_string) {
                    Ok(value) => value,
                    Err(..) => panic!(),
                };
                _ = api::update(backend_url, token, password_hash, &json_value);
                if entry_return.1 == true {
                    _ = api::logout(backend_url, token);
                }
            }
            Ok((200, None)) => {
                let new_json: Entry = view::display_data_empty();
                let new_json = vec![new_json];
                let new_json = serialize_json(&new_json).unwrap();
                let json_string = new_json.as_str();
                let new_json: Value = match serde_json::from_str(json_string) {
                    Ok(value) => value,
                    Err(..) => panic!(),
                };
                _ = api::update(backend_url, token, password_hash, &new_json);
            }
            Ok((401, _)) => {
                view::update_error(401);
                return;
            }
            Ok((500, _)) => {
                view::update_error(500);
                return;
            }
            Ok((status_code, _)) => {
                view::update_error(status_code as i16);
                return;
            }
            Err(_e) => {
                view::update_error(418);
                return;
            }
        }
    }
}

pub fn get_uris(json_entries: Vec<Entry>) -> Result<Vec<String>, Error> {
    //array für die Speicherung der uris anlegen
    let mut uris = Vec::new();
    for element in json_entries {
        for uri in element.login.uris {
            uris.push(uri.uri);
        }
    }
    Ok(uris)
}

pub fn deserialize_json(json_data: &Value) -> Result<Vec<Entry>, Error> {
    let entries = serde_json::from_value(json_data.clone());
    match entries {
        Ok(entry_list) => Ok(entry_list),
        Err(e) => match e.classify() {
            Category::Io => {
                debug!("Failed to read or write bytes on an I/O stream");
                Err(e)
            }
            Category::Syntax => {
                debug!("Input is not syntactically valid JSON");
                Err(e)
            }
            Category::Data => {
                debug!("Input data is semantically incorrect");
                Err(e)
            }
            Category::Eof => {
                debug!("Unexpected end of the input data");
                Err(e)
            }
        },
    }
}

pub fn serialize_json(entries: &Vec<Entry>) -> Option<String> {
    let json = match serde_json::to_string(&entries) {
        Ok(value) => value,
        Err(_) => {
            view::serialization_error();
            return None;
        }
    };
    Some(json)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uri {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
    pub uris: Vec<Uri>,
    pub username: Option<String>,
    pub password: String,
    pub totp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Entry {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub login: Login,
}

pub fn validate_string_length(string: &str) -> bool {
    let capacity: usize = 200;
    string.len() <= capacity
}
