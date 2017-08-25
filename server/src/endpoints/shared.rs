use hyper::header::ContentType;
use serde;
use serde_json;
use hyper::server::{Response};
use hyper::StatusCode;

fn error_response(error_message: String) -> Response {
  Response::new()
    .with_status(StatusCode::InternalServerError)
    .with_body(error_message)
}

pub fn to_json_response<T>(body: T) -> Response 
where T: serde::Serialize {
  match serde_json::to_string(&body) {
    Ok(result) => 
      Response::new()
        .with_body(result)
        .with_header(ContentType::json()),
    _ => error_response(String::from("json flopp"))
  }
}
