use serde_json;
use hyper::server::{Request, Response};
use hyper::header::ContentType;
use hyper;
use futures;
use http_method::HttpMethod;
use game_screen;
use futures::{Future, Stream, BoxFuture};

#[derive(Deserialize)]
pub struct LoginBody {
  pub username: String
}

pub fn get_root(_: Request) -> BoxFuture<Response, hyper::Error> {
  let response = game_screen::GameScreen{
    message: String::from("Welcome to Hero Quest! What is your name?"),
    actions: vec!(
      game_screen::GameAction {
        name: String::from("Login form"),
        method: HttpMethod::Post,
        fields: vec![
          game_screen::Field {
            name: String::from("username"),
            value: game_screen::FieldValue::Text(None)
          }
        ],
        link: String::from("http://localhost:4000/user"),
      },
    )
  };

  let body = match serde_json::to_string(&response) {
    Ok(result) => result,
    _ => String::from("Flopp")
  };

  Box::new(futures::future::ok(Response::new().with_body(body).with_header(ContentType::json())))
}

pub fn post_user(request: Request) -> BoxFuture<Response, hyper::Error> {
  Box::new(request.body().concat2().and_then(|body| {
    let login_body = serde_json::from_slice::<LoginBody>(&body).unwrap();

    let response = game_screen::GameScreen{
    message: format!("{} is standing in a small village.", login_body.username),
    actions: vec!(
      game_screen::GameAction {
        name: String::from("Check quests"),
        method: HttpMethod::Get,
        fields: vec![],
        link: String::from("http://localhost:4000/quests"),
      },
    )
  };
  
  let body = match serde_json::to_string(&response) {
    Ok(result) => result,
    _ => String::from("Flopp")
  };
  futures::future::ok(Response::new().with_body(body).with_header(ContentType::json()))
  }))
}