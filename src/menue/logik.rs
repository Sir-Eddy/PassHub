use serde_json::Error;
use serde_json::Value;
use serde_json::error::Category;
use serde::{Serialize, Deserialize};
use super::{api, view};
use log::debug;




pub fn main_menue(backend_url: &String, token: &String, password_hash: &String) {

    loop {
    // Get the passwords from the backend
    let json_data_result = api::fetch(&backend_url, &token, &password_hash);

    match json_data_result {
        Ok((200, Some(json_data))) => {
            let entries = view::display_data(&json_data);

            let json_string = serialize_json(&entries.unwrap()).unwrap();
            let json_string = json_string.as_str();
            let json_value: Value = match serde_json::from_str(&json_string)
            {
                Ok(value) => value,
                Err(..) => panic!(),
            };
            _ = api::update(&backend_url, &token, &password_hash, &json_value);
            _ = api::logout(&backend_url, &token);
        }
        Ok((200, None)) => {
            let _ = view::display_data_empty();
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


pub fn deserialize_json(json_data: &Value)->Result<Vec<Entry>, Error>{
    let entries  = serde_json::from_value(json_data.clone());
    match entries {
        Ok(entry_list)=> {Ok(entry_list)},
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

pub fn serialize_json(entries: &Vec<Entry>)-> Option<String>{
    let json = match serde_json::to_string(&entries){
        Ok(value) => value,
        Err(_) => {view::serialization_error();
        return None;
        },
    };
    return Some(json);
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


pub fn validate_string_length(string: &String)-> bool {
    let capacity: usize = 200;
    if string.len() > capacity {
        return false;
    }
    else {
        return true;
    }
}