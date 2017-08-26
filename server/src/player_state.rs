use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use game_state;
use rocket::http::Cookies;
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard, Arc};

pub struct PlayerState {
  username: String
}

pub struct ThreadSafePlayerState {
  pub player_state: Arc<RwLock<PlayerState>>
}

impl ThreadSafePlayerState {
  pub fn new(player: Arc<RwLock<PlayerState>>) -> ThreadSafePlayerState {
    ThreadSafePlayerState{ player_state: player }
  }

  pub fn read(&self) -> RwLockReadGuard<PlayerState> {
    return self.player_state.read().unwrap()
  }

  pub fn write(&self) -> RwLockWriteGuard<PlayerState> {
    return self.player_state.write().unwrap()
  }
}

impl PlayerState {
  pub fn new(username: String) -> PlayerState {
    PlayerState { username: username }
  } 

  pub fn username(&self) -> &String {
    return &self.username;
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for ThreadSafePlayerState {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ThreadSafePlayerState, ()> {
        let cookies = request.guard::<Cookies>()?;

        match cookies.get("id") {
          Some(cookie) => {
            let state = request.guard::<State<game_state::ThreadSafeGameState>>()?;
            let lock = state.read();
            match lock.get_player(cookie.value()) {
                Some(player) => {
                  Outcome::Success(ThreadSafePlayerState::new(player))
                },
                None => Outcome::Failure((Status::ServiceUnavailable, ()))
            }
          },
          None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}