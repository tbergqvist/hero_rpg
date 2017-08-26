use game_screen;
use http_method::HttpMethod;
use game_state::{TsGameState};
use player_state::TsPlayerState;
use quest::Quest;

pub fn get_quests_screen(game_state: &TsGameState, player_state: &TsPlayerState) -> game_screen::GameScreen {
  let quests = get_acceptable_quests(&game_state, &player_state);

  let actions = quests.iter().map(|quest| {
    game_screen::GameAction {
      name: quest.name().clone(),
      method: HttpMethod::Post,
      link: format!("http://localhost:4000/quests/{}", quest.id()),
      fields: vec![]
    }
  }).collect();

  game_screen::GameScreen{
    message: format!("Here are all the quests!"),
    actions: actions
  }
}

fn get_acceptable_quests(game_state: &TsGameState, player_state: &TsPlayerState) -> Vec<Quest> {
  game_state.read()
  .get_quests()
  .iter()
  .filter(|quest| player_state.read().accepted_quests().iter().find(|accepted_quest| accepted_quest.quest_id() == quest.id()).is_none())
  .map(|quest| quest.clone())
  .collect()
}