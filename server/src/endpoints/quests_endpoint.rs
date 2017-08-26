use endpoints::commands;
use game_screen;
use rocket_contrib::Json;
use rocket::State;
use rocket;


use game_state::TsGameState;
use player_state::TsPlayerState;

#[get("/quests")]
pub fn get_quests(game_state: State<TsGameState>, player_state: TsPlayerState) -> Json<game_screen::GameScreen> {
  Json(
    commands::get_quests_screen(&game_state, &player_state)
  )
}

#[post("/quests/<id>")]
pub fn accept_quest(id: i32, game_state: State<TsGameState>, player_state: TsPlayerState) -> Result<Json<game_screen::GameScreen>, rocket::response::Failure> {
  match game_state.read().get_quest(id) {
    Some(quest) => {
      player_state.write().accept_quest(quest);
      Ok(
        Json(
          commands::get_quests_screen(&game_state, &player_state)
        )
      )
    },
    None => Err(rocket::response::Failure(rocket::http::Status::BadRequest))
  }
}