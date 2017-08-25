use game_screen;
use http_method::HttpMethod;

pub fn get_quests() -> game_screen::GameScreen {
  game_screen::GameScreen{
    message: format!("Here are all the quests!"),
    actions: vec![
      game_screen::GameAction {
        name: String::from("Kill 10 goblins"),
        method: HttpMethod::Post,
        link: String::from("http://localhost:4000/quests/1"),
        fields: vec![]
      }
    ]
  }
}