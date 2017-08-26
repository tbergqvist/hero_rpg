use endpoints::commands;
use game_screen;
use rocket_contrib::Json;
use player_state::ThreadSafePlayerState;

#[get("/quests")]
pub fn get_quests(player_state: ThreadSafePlayerState) -> Json<game_screen::GameScreen> {
  println!("{} is getting quests!", player_state.read().username());
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