use hyper;
use hyper::server::{Request, Response};
use futures::BoxFuture;
use futures;
use endpoints::commands;
use endpoints::shared;

pub fn get_quests(_: Request) -> BoxFuture<Response, hyper::Error> {
  Box::new(
    futures::future::ok(
      shared::to_json_response(
        commands::get_quests::get_quests()
      )
    )
  )
}

pub fn accept_quest(_: Request) -> BoxFuture<Response, hyper::Error> {
  Box::new(
    futures::future::ok(
      shared::to_json_response(
        commands::accept_quest::accept_quest()
      )
    )
  )
}