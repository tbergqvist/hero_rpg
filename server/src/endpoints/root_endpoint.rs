use rocket_contrib::Json;

use endpoints::commands;
use game_screen;

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
pub fn login(body: Json<LoginBody>) -> Json<game_screen::GameScreen> {
  Json(
    commands::get_village::get_village(&body.username)
  )
}