extern crate unicase;

use root_endpoint;
use quests_endpoint;

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
  pub callback: Box<Fn (Request, &Router) -> BoxFuture<Response, hyper::Error>>
}

pub struct Router {
  quests_endpoint: quests_endpoint::QuestsEndpoint,
  root_endpoint: root_endpoint::RootEndpoint,
  routes: Vec<Route>
}

impl Router {
  pub fn new() -> Router {
    let quests_endpoint = quests_endpoint::QuestsEndpoint{};
    let root_endpoint = root_endpoint::RootEndpoint{};

    Router {
      routes: vec![
        Route { path: "/quests", method: Method::Get, callback: Box::new(|request, router|router.quests_endpoint.get_quests(request)) },
        Route { path: "/", method: Method::Get, callback: Box::new(|request, router|router.root_endpoint.get_root(request)) },
        Route { path: "/user", method: Method::Post, callback: Box::new(|request, router|router.root_endpoint.post_user(request)) }
      ],
      quests_endpoint: quests_endpoint,
      root_endpoint: root_endpoint,
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
        let response: BoxFuture<Response, hyper::Error> = (route.callback)(request, &self);
        return Box::new(response.and_then(|response|{
          Ok(response.with_header(AccessControlAllowOrigin::Any))
        }));
        //return response.with_header(AccessControlAllowOrigin::Any);
      }
    }

    Box::new(futures::future::ok(Response::new().with_status(StatusCode::NotFound)))
  }
}