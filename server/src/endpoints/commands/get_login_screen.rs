use gui_item::{GuiItem, FieldItem};
use http_method::HttpMethod;
use config::Config;

pub fn get_login_screen(config: &Config) -> Vec<GuiItem> {
  vec![
    GuiItem::Text{value: String::from("Welcome to Hero Quest! What is your name?")},
    GuiItem::Form{
      name: String::from("Login form"),
      method: HttpMethod::Post,
      fields: vec![
        FieldItem::TextInput{ name: String::from("username"), value: None}
      ],
      link: format!("{}/login", config.base_url),
    },
  ]
}