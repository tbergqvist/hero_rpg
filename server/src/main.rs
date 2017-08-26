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
mod game_state;
mod player_state;
mod quest;
mod enemy;
mod thread_safe;

#[macro_use]
extern crate serde_derive;

use endpoints::{root_endpoint, quests_endpoint};

fn main() {
  // You can also deserialize this
  let cors = rocket_cors::Cors {
      allow_credentials: true,
      ..Default::default()
  };

  rocket::ignite()
    .mount("/", routes![
      root_endpoint::get_login_screen,
      root_endpoint::get_village,
      root_endpoint::login,
      quests_endpoint::get_quests,
      quests_endpoint::accept_quest
    ])
    .attach(cors)
    .manage(thread_safe::Ts::new(game_state::GameState::new()))
  .launch();
}
