use endpoints::commands;
use rocket_contrib::Json;
use rocket::State;
use rocket;

use game_state::TsGameState;
use player_state::TsPlayerState;
use config::Config;
use gui_item::{GuiItem};

#[get("/", rank=1)]
pub fn get_village(player_state: TsPlayerState, config: State<Config>) -> Json<Vec<GuiItem>> {
  Json(
    commands::get_village::get_village(&player_state, &config)
  )
}

#[get("/quests")]
pub fn get_quests(game_state: State<TsGameState>, player_state: TsPlayerState, config: State<Config>) -> Json<Vec<GuiItem>> {
  Json(
    commands::get_quests_screen(&game_state, &player_state, &config)
  )
}

#[post("/quests/<id>")]
pub fn accept_quest(id: i32, game_state: State<TsGameState>, player_state: TsPlayerState, config: State<Config>) -> Result<Json<Vec<GuiItem>>, rocket::response::Failure> {
  match game_state.read().get_quest(id) {
    Some(quest) => {
      player_state.write().accept_quest(quest);
      Ok(
        Json(
          commands::get_quests_screen(&game_state, &player_state, &config)
        )
      )
    },
    None => Err(rocket::response::Failure(rocket::http::Status::BadRequest))
  }
}