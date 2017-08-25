extern crate unicase;

use endpoints::{quests_endpoint, root_endpoint};

use futures;
use futures::{Future, BoxFuture};
use hyper;
use hyper::server::{Request, Response};
use hyper::{Method, StatusCode};
use hyper::header::AccessControlAllowOrigin;
use hyper::header::AccessControlAllowHeaders;

extern crate serde;

pub struct Route {
  pub path: &'static str,
  pub method: Method,
  pub callback: fn (Request) -> BoxFuture<Response, hyper::Error>
}

pub struct Router {
  routes: Vec<Route>
}

impl Router {
  pub fn new() -> Router {
    Router {
      routes: vec![
        Route { path: "/quests", method: Method::Get, callback: quests_endpoint::get_quests },
        Route { path: "/quests/1", method: Method::Post, callback: quests_endpoint::accept_quest },
        Route { path: "/", method: Method::Get, callback: root_endpoint::get_login_screen },
        Route { path: "/user", method: Method::Post, callback: root_endpoint::login }
      ],
    }
  }

  pub fn handle_request(&self, request: Request) -> BoxFuture<Response, hyper::Error> {
    if request.method() == &Method::Options {
      return Box::new(
        futures::future::ok(
          Response::new()
          .with_header(AccessControlAllowOrigin::Any)
          .with_header(AccessControlAllowHeaders(vec![unicase::Ascii::new(String::from("Content-Type"))]))
          )
        );
    }

    for route in &self.routes {
      if request.path() == route.path && request.method() == &route.method {
        let response: BoxFuture<Response, hyper::Error> = (route.callback)(request);
        return Box::new(response.and_then(|response|{
          Ok(response.with_header(AccessControlAllowOrigin::Any))
        }));
      }
    }

    Box::new(futures::future::ok(Response::new().with_status(StatusCode::NotFound)))
  }
}