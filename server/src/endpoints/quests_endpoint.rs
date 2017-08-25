use endpoints::commands;
use game_screen;
use rocket_contrib::Json;

#[get("/quests")]
pub fn get_quests() -> Json<game_screen::GameScreen> {
  Json(
    commands::get_quests::get_quests()
  )
}

#[post("/quests/<id>")]
pub fn accept_quest(id: i32) -> Json<game_screen::GameScreen> {
  Json(
    commands::accept_quest::accept_quest()
  )
}