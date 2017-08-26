use game_screen;
use http_method::HttpMethod;
use player_state::TsPlayerState;

pub fn get_village(player_state: &TsPlayerState) -> game_screen::GameScreen {
  game_screen::GameScreen{
    message: format!("{} is standing in a small village.", player_state.read().username()),
    actions: vec!(
      game_screen::GameAction {
        name: String::from("Check quests"),
        method: HttpMethod::Get,
        fields: vec![],
        link: String::from("http://localhost:4000/quests"),
      },
    )
  }
}