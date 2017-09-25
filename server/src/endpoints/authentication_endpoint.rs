use rocket::State;
use rocket_contrib::Json;
use rocket::http::Cookies;
use rocket::http::Cookie;

use endpoints::commands;
use game_state::TsGameState;
use config::Config;
use gui_item::{GuiItem};

#[derive(Deserialize)]
pub struct LoginBody {
  pub username: String
}

#[get(path = "/", rank=2)]
pub fn get_login_screen(config: State<Config>) -> Json<Vec<GuiItem>> {
  Json(
    commands::get_login_screen::get_login_screen(&config)
  )
}

#[post("/login", data = "<body>")]
pub fn login(body: Json<LoginBody>, state: State<TsGameState>, mut cookies: Cookies, config: State<Config>) -> Json<Vec<GuiItem>> {
  let player_state = state.write().login_player(&body.username);

  let cookie = Cookie::build("id", body.username.clone())
    .http_only(true)
    .finish();
    
  cookies.add(cookie);
  Json(
    commands::get_village::get_village(&player_state, &config)
  )
}

#[post("/logout")]
pub fn logout(config: State<Config>, mut cookies: Cookies) -> Json<Vec<GuiItem>> {
  cookies.remove(Cookie::named("id"));
  get_login_screen(config)
}