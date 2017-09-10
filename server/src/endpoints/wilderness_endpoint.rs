use game_screen;
use rocket_contrib::Json;
use rocket::State;

use game_state::TsGameState;
use player_state::{TsPlayerState, BattleState};
use config::Config;
use endpoints::commands::get_village;
use http_method::HttpMethod;

use rand;
use rand::Rng;

#[get("/wilderness")]
pub fn enter_wilderness(game_state: State<TsGameState>, player_state: TsPlayerState, config: State<Config>) -> Json<game_screen::GameScreen> {
  let game_state_lock = game_state.read();
  let enemy = rand::thread_rng().choose(game_state_lock.get_enemy_templates()).unwrap().clone();
  let mut player_lock = player_state.write();
  let battle = player_lock.init_battle(enemy);

  Json(
    game_screen::GameScreen{
    message: format!("You are fighting a {}! HP: {}/{}", battle.enemy.name, battle.enemy.current_hp, battle.enemy.max_hp),
    actions: vec!(
      game_screen::GameAction {
        name: String::from("Attack"),
        method: HttpMethod::Post,
        link: format!("{}/fight", config.base_url),
        fields: vec![]
      },
    )
  })
}

#[post("/fight")]
pub fn fight(player_state: TsPlayerState, config: State<Config>) -> Json<game_screen::GameScreen> {
  let mut player_lock = player_state.write();
  let state = player_lock.fight();

  Json(
    match state {
      BattleState::Fight(battle) => {
        game_screen::GameScreen{
          message: format!("You are fighting a {}! HP: {}/{}", battle.enemy.name, battle.enemy.current_hp, battle.enemy.max_hp),
          actions: vec!(
            game_screen::GameAction {
              name: String::from("Attack"),
              method: HttpMethod::Post,
              link: format!("{}/fight", config.base_url),
              fields: vec![]
            },
          )
        }
      },
      BattleState::End(reward) => {
        game_screen::GameScreen{
          message: format!("You win!"),
          actions: vec!(
            game_screen::GameAction {
              name: String::from("Back to town"),
              method: HttpMethod::Get,
              link: format!("{}/", config.base_url),
              fields: vec![]
            },
          )
        }
      }
    }
  )
  //let battle = player_lock.current_battle.unwrap();
  
  
  /*
  match (battle) {
    fight(battle) => {
      Json(
      game_screen::GameScreen{
      message: format!("You are fighting a {}! HP: {}/{}", battle.enemy.name, battle.enemy.current_hp, battle.enemy.max_hp),
      actions: vec!(
        game_screen::GameAction {
          name: String::from("Attack"),
          method: HttpMethod::Post,
          link: format!("{}/fight", config.base_url),
          fields: vec![]
        },
      )
    })
    },
    end(reward) => {
      Json(
        game_screen::GameScreen{
        message: format!("You won! You get {} gold!", reward.gold),
        actions: vec!(
          game_screen::GameAction {
            name: String::from("Attack"),
            method: HttpMethod::Post,
            link: format!("{}/fight", config.base_url),
            fields: vec![]
          },
      )
    })
  }
  }
  */
}