#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate serde;
extern crate serde_json;

mod endpoints;
mod game_screen;
mod http_method;

#[macro_use]
extern crate serde_derive;

use endpoints::{root_endpoint, quests_endpoint};


fn main() {
  let options = rocket_cors::Cors {
    ..Default::default()
  };

  rocket::ignite()
    .mount("/", routes![
      root_endpoint::get_login_screen,
      root_endpoint::login,
      quests_endpoint::get_quests,
      quests_endpoint::accept_quest
    ])
    .attach(options)
  .launch();
}
