use gui_item::{GuiItem};

use rocket_contrib::Json;
use rocket::State;

use rand;
use rand::Rng;

use game_state::TsGameState;
use player_state::{TsPlayerState, BattleState};
use config::Config;
use http_method::HttpMethod;
use common_actions;

fn get_fight_action(base_url: &String) -> GuiItem {
  GuiItem::Form {
    name: String::from("Attack"),
    method: HttpMethod::Post,
    link: format!("{}/fight", base_url),
    fields: vec![]
  }
}

#[get("/wilderness")]
pub fn enter_wilderness(game_state: State<TsGameState>, player_state: TsPlayerState, config: State<Config>) -> Json<Vec<GuiItem>> {
  let game_state_lock = game_state.read();
  let enemy = rand::thread_rng().choose(game_state_lock.get_enemy_templates()).unwrap().clone();
  let mut player_lock = player_state.write();
  let battle = player_lock.init_battle(enemy);

  Json(
    vec![
      GuiItem::Text{value: format!("You are fighting a {}! HP: {}/{}", battle.enemy.name, battle.enemy.current_hp, battle.enemy.max_hp)},
      get_fight_action(&config.base_url),
    ]
  )
}

#[post("/fight")]
pub fn fight(player_state: TsPlayerState, config: State<Config>) -> Json<Vec<GuiItem>> {
  let mut player_lock = player_state.write();
  let state = player_lock.fight();

  Json(
    match state {
      BattleState::Fight(battle) => {
        vec![
          GuiItem::Text{value: format!("You are fighting a {}! HP: {}/{}", battle.enemy.name, battle.enemy.current_hp, battle.enemy.max_hp)},
          get_fight_action(&config.base_url),
        ]
      },
      BattleState::End(reward) => {
        vec![
          GuiItem::Text{value: String::from("You win")},
          common_actions::back_to_village_form(&config.base_url)
        ]
      }
    }
  )
}