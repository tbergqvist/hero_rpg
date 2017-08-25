use game_screen;
use http_method::HttpMethod;

pub fn get_village(username: &str) -> game_screen::GameScreen {
  game_screen::GameScreen{
    message: format!("{} is standing in a small village.", username),
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