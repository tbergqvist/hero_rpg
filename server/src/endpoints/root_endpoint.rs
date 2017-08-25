use rocket::State;
use rocket_contrib::Json;
use rocket::http::Cookies;
use rocket::http::Cookie;

use endpoints::commands;
use game_screen;
use game_state::ThreadSafeGameState;

#[derive(Deserialize)]
pub struct LoginBody {
  pub username: String
}

#[get("/")]
pub fn get_login_screen() -> Json<game_screen::GameScreen> {
  Json(
    commands::get_login_screen::get_login_screen()
  )
}

#[post("/login", data = "<body>")]
pub fn login(body: Json<LoginBody>, state: State<ThreadSafeGameState>, mut cookies: Cookies) -> Json<game_screen::GameScreen> {
  state.write().login_player(&body.username);

  let cookie = Cookie::build("id", body.username.clone())
    .http_only(true)
    .finish();
    
  cookies.add(cookie);
  Json(
    commands::get_village::get_village(&body.username)
  )
}