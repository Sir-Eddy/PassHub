use super::{api, view};

pub fn delete(backend_url: &String, jwt_token: &String) {
    let status_code = api::delete(backend_url, jwt_token);

    match status_code.expect("Delete: Error occurred deleting the account") {
        200 => view::draw_delete_screen(),
        400 | 401 | 500 => view::error(),
        _ => view::error(),
    }
}
