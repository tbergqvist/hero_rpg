use std::collections::HashMap;
use player_state::{PlayerState, TsPlayerState};
use quest::{Quest, create_quests};
use enemy::{Enemy, create_enemies};
use thread_safe::Ts;

pub type TsGameState = Ts<GameState>;

pub struct GameState {
  enemies: Vec<Enemy>,
  quests: Vec<Quest>,
  players: HashMap<String, TsPlayerState>
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      enemies: create_enemies(),
      quests: create_quests(),
      players: HashMap::new() 
    }
  }

  pub fn login_player(&mut self, name: &str) -> TsPlayerState {
    let name = String::from(name);
    let player_ptr = self.players.entry(name.clone()).or_insert(TsPlayerState::new(PlayerState::new(name)));
    return player_ptr.clone();
  }

  pub fn get_player(&self, name: &str) -> Option<TsPlayerState> {
    match self.players.get(name) {
      Some(lock) => Some(lock.clone()),
      None => None
    }
  }

  pub fn get_quests(&self) -> &Vec<Quest> {
    &self.quests
  }

  pub fn get_quest(&self, quest_id: i32) -> Option<&Quest> {
    self.quests.iter().find(|quest| quest.id() == quest_id)
  }

  pub fn get_enemy_templates(&self) -> &Vec<Enemy> {
    &self.enemies
  }
}