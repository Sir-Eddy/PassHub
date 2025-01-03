use reqwest::blocking::Client;



pub fn delete(backend_url: &String, jwt_token: &String)->Result<u16, Box<std::din::error::Error>>{
    let client = Client::new();

    let request_url = format!("{}/api/v1/account/delete", backend_url);

    let response = client
    .get(&request_url)
    .header("Authorization", format!("Bearer {}", jwt_token))
    .send()?;

    let status_code = response.status().as_u16();
    
    match status_code{
        200 => Ok(status_code),
        400 | 401 | 500 => Ok(status_code),
    }
}