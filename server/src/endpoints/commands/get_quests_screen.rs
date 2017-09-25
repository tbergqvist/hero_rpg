use gui_item::{GuiItem};
use common_actions;

use http_method::HttpMethod;
use game_state::{TsGameState, GameState};
use player_state::TsPlayerState;
use quest::Quest;
use config::Config;
use std::iter;

fn quest_to_gui(quest: &Quest, config: &Config) -> GuiItem {
  GuiItem::Form {
    name: quest.name().clone(),
    method: HttpMethod::Post,
    link: format!("{}/quests/{}", config.base_url, quest.id()),
    fields: vec![]
  }
}

fn get_acceptable_quests<'a>(game_state: &'a GameState, player_state: &'a TsPlayerState, config: &'a Config) -> Box<Iterator<Item = GuiItem> + 'a> {
  Box::new(
    game_state.get_quests()
    .iter()
    .filter(move |quest| player_state.read().accepted_quests().iter().find(|accepted_quest| accepted_quest.quest_id() == quest.id()).is_none())
    .map(move |quest| {
      quest_to_gui(quest, config)
    })
  )
}

pub fn get_quests_screen(game_state: &TsGameState, player_state: &TsPlayerState, config: &Config) -> Vec<GuiItem> {
  let locked_game_state = game_state.read();

  let val = iter::once(GuiItem::Text{value: String::from("Here are all the quests!")})
  .chain(get_acceptable_quests(&locked_game_state, &player_state, &config))
  .chain(iter::once(common_actions::back_to_village_form(&config.base_url)))
  .collect();
  val
}