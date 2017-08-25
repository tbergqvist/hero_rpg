use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use game_state;
use rocket::http::Cookies;

pub struct PlayerState {
}

impl Clone for PlayerState {
  fn clone(&self) -> PlayerState {
    PlayerState::new()
  }
}

impl PlayerState {
  pub fn new() -> PlayerState {
    PlayerState {}
  } 
}

impl<'a, 'r> FromRequest<'a, 'r> for PlayerState {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<PlayerState, ()> {
        let cookies = request.guard::<Cookies>()?;
        match cookies.get("id") {
          Some(cookie) => {
            let state = request.guard::<State<game_state::ThreadSafeGameState>>()?;
            let lock = state.read();
            match lock.get_player(cookie.value()) {
                Some(player) => {
                  Outcome::Success(player.clone())
                },
                None => Outcome::Failure((Status::ServiceUnavailable, ()))
            }
          },
          None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}