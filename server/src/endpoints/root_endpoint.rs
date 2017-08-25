use serde_json;
use hyper::server::{Request, Response};
use hyper;
use futures;
use futures::{Future, Stream, BoxFuture};

use endpoints::commands;
use endpoints::shared;

use hyper::StatusCode;

#[derive(Deserialize)]
pub struct LoginBody {
  pub username: String
}

pub fn get_login_screen(_: Request) -> BoxFuture<Response, hyper::Error> {
  Box::new(
    futures::future::ok(
      shared::to_json_response(
        commands::get_login_screen::get_login_screen()
      )
    )
  )
}

pub fn json_error_response(error: serde_json::Error) -> Response {
  Response::new()
    .with_status(StatusCode::BadRequest)
    .with_body(format!("{}", error))
}

pub fn login(request: Request) -> BoxFuture<Response, hyper::Error> {
  Box::new(request.body().concat2().and_then(|body| {
    Ok(match serde_json::from_slice::<LoginBody>(&body) {
      Ok(result) => 
        shared::to_json_response(
          commands::get_village::get_village(&result.username)
        ),
      Err(error) => 
        json_error_response(error)
    })
  }))
}