use serde_json;

use game_screen; 
use hyper;
use hyper::server::{Request, Response};
use hyper::header::ContentType;
use futures::BoxFuture;
use futures;
use http_method::HttpMethod;

pub fn get_quests(_: Request) -> BoxFuture<Response, hyper::Error> {
  let response = game_screen::GameScreen{
    message: format!("Here are all the quests!"),
    actions: vec![
      game_screen::GameAction {
        name: String::from("Kill 10 goblins"),
        method: HttpMethod::Post,
        link: String::from("http://localhost:4000/quests/1"),
        fields: vec![]
      }
    ]
  };

  let body = match serde_json::to_string(&response) {
    Ok(result) => result,
    _ => String::from("Flopp")
  };

  Box::new(futures::future::ok(Response::new().with_body(body).with_header(ContentType::json())))
}

pub fn accept_quest(_: Request) -> BoxFuture<Response, hyper::Error> {
  let response = game_screen::GameScreen{
    message: format!("Here are all the quests! 1 Quest accepted!"),
    actions: vec![
      game_screen::GameAction {
        name: String::from("Back to village"),
        method: HttpMethod::Get,
        link: String::from("http://localhost:4000/"),
        fields: vec![]
      }
    ]
  };

  let body = match serde_json::to_string(&response) {
    Ok(result) => result,
    _ => String::from("Flopp")
  };

  Box::new(futures::future::ok(Response::new().with_body(body).with_header(ContentType::json())))
}