extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;

mod endpoints;
mod router;
mod game_screen;
mod http_method;

#[macro_use]
extern crate serde_derive;

use hyper::server::{Http, Request, Response, Service};

use futures::BoxFuture;

struct WebServer {
  router: router::Router
}

impl  WebServer {
  fn new() -> WebServer {
    let rout = router::Router::new();

    WebServer{ router: rout}
  }
}

impl Service for WebServer {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = BoxFuture<Self::Response, Self::Error>;

  fn call(&self, req: Request) -> Self::Future {
    self.router.handle_request(req)
  }
}

fn main() {
  let addr = "127.0.0.1:4000".parse().unwrap();
  let server = Http::new().bind(&addr, || Ok(WebServer::new())).unwrap();
  server.run().unwrap();
}
