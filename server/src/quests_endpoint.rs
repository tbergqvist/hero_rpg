use serde_json;

use game_screen; 
use hyper;
use hyper::server::{Request, Response};
use hyper::header::ContentType;
use futures::BoxFuture;
use futures;

pub struct QuestsEndpoint {
}

impl QuestsEndpoint {
  pub fn get_quests(&self, _: Request) -> BoxFuture<Response, hyper::Error> {
    let response = game_screen::GameScreen{
      message: format!("No quest!"),
      actions: vec![]
    };

    let body = match serde_json::to_string(&response) {
      Ok(result) => result,
      _ => String::from("Flopp")
    };

    Box::new(futures::future::ok(Response::new().with_body(body).with_header(ContentType::json())))
  }
}