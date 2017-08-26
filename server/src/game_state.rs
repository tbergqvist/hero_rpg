use std::collections::HashMap;
use std::sync::Arc;
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use player_state::{PlayerState};

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
  players: HashMap<String, Arc<RwLock<PlayerState>>>
}

impl GameState {
  pub fn new() -> GameState {
    GameState{ players: HashMap::new() }
  }

  pub fn login_player(&mut self, name: &str) -> Arc<RwLock<PlayerState>> {
    let name = String::from(name);
    let player_ptr = self.players.entry(name.clone()).or_insert(Arc::new(RwLock::new(PlayerState::new(name))));
    return player_ptr.clone();
  }

  pub fn get_player(&self, name: &str) -> Option<Arc<RwLock<PlayerState>>> {
    match self.players.get(name) {
      Some(lock) => Some(lock.clone()),
      None => None
    }
  }
}