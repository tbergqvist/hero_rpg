#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate serde;
extern crate rand;

mod endpoints;
mod game_screen;
mod http_method;
mod game_state;
mod player_state;
mod quest;
mod enemy;
mod thread_safe;
mod config;
mod battle;

#[macro_use]
extern crate serde_derive;

use endpoints::{authentication_endpoint, village_endpoint, wilderness_endpoint};
use rocket::fairing::AdHoc;

fn main() {
  // You can also deserialize this
  let cors = rocket_cors::Cors {
      allow_credentials: true,
      ..Default::default()
  };

  rocket::ignite()
    .mount("/", routes![
      authentication_endpoint::get_login_screen,
      authentication_endpoint::login,
      authentication_endpoint::logout,
      village_endpoint::get_village,
      village_endpoint::get_quests,
      village_endpoint::accept_quest,
      wilderness_endpoint::enter_wilderness,
      wilderness_endpoint::fight,
    ])
    .attach(cors)
    .attach(AdHoc::on_attach(|rocket| {
      let config = {
        let config = rocket.config();
        let base_url = format!("http://{}:{}", config.address, config.port);
        config::Config{base_url: base_url}
      };
      Ok(rocket.manage(config))
    }))
    .manage(thread_safe::Ts::new(game_state::GameState::new()))
  .launch();
}
