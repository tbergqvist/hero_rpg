use game_screen;
use http_method::HttpMethod;
use config::Config;

pub fn get_login_screen(config: &Config) -> game_screen::GameScreen {
  game_screen::GameScreen{
    message: String::from("Welcome to Hero Quest! What is your name?"),
    actions: vec!(
      game_screen::GameAction {
        name: String::from("Login form"),
        method: HttpMethod::Post,
        fields: vec![
          game_screen::Field {
            name: String::from("username"),
            value: game_screen::FieldValue::Text(None)
          }
        ],
        link: format!("{}/login", config.base_url),
      },
    )
  }
}