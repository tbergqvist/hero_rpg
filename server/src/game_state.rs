use std::collections::HashMap;
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use player_state::PlayerState;

pub struct ThreadSafeGameState {
  pub game_state: RwLock<GameState>
}

impl ThreadSafeGameState {
  pub fn new() -> ThreadSafeGameState {
    ThreadSafeGameState{ game_state: RwLock::new(GameState::new()) }
  }

  pub fn read(&self) -> RwLockReadGuard<GameState> {
    return self.game_state.read().unwrap()
  }

  pub fn write(&self) -> RwLockWriteGuard<GameState> {
    return self.game_state.write().unwrap()
  }
}

pub struct GameState {
  pub players: HashMap<String, PlayerState>
}

impl GameState {
  pub fn new() -> GameState {
    GameState{ players: HashMap::new() }
  }

  pub fn login_player(&mut self, name: &str) -> &PlayerState {
    self.players.entry(String::from(name)).or_insert(PlayerState::new())
  }

  pub fn get_player(&self, name: &str) -> Option<&PlayerState> {
    self.players.get(name)
  }
}