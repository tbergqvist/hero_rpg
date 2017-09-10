use game_screen;
use http_method::HttpMethod;
use player_state::TsPlayerState;
use config::Config;

pub fn get_village(player_state: &TsPlayerState, config: &Config) -> game_screen::GameScreen {
  game_screen::add_logout(
    game_screen::GameScreen{
      message: format!("{} is standing in a small village.", player_state.read().username()),
      actions: vec!(
        game_screen::GameAction {
          name: String::from("Check quests"),
          method: HttpMethod::Get,
          fields: vec![],
          link: format!("{}/quests", config.base_url),
        },
        game_screen::GameAction {
          name: String::from("Enter wilderness"),
          method: HttpMethod::Get,
          fields: vec![],
          link: format!("{}/wilderness", config.base_url),
        }
      )
    }, &config.base_url
  )
}