use gui_item::{GuiItem};
use common_actions;
use http_method::HttpMethod;
use player_state::TsPlayerState;
use config::Config;
use std::iter;

fn enter_wilderness_form(base_url: &String) -> GuiItem {
  GuiItem::Form {
    name: String::from("Enter wilderness"),
    method: HttpMethod::Get,
    fields: vec![],
    link: format!("{}/wilderness", base_url)
  }
}

fn check_quests_form(base_url: &String) -> GuiItem {
  GuiItem::Form {
    name: String::from("Check quests"),
    method: HttpMethod::Get,
    fields: vec![],
    link: format!("{}/quests", base_url)
  }
}

pub fn get_village(player_state: &TsPlayerState, config: &Config) -> Vec<GuiItem> {
  iter::once(GuiItem::Text{value: format!("{} is standing in a small village.", player_state.read().username())})
  .chain(iter::once(check_quests_form(&config.base_url)))
  .chain(iter::once(enter_wilderness_form(&config.base_url)))
  .chain(iter::once(common_actions::get_logout_form(&config.base_url)))
  .collect()
}