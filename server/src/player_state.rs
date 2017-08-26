use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use game_state;
use quest::Quest;
use rocket::http::Cookies;
use thread_safe::Ts;

pub struct AcceptedQuest {
  quest_id: i32,
  name: String,
  monsters_killed: i32
}

impl AcceptedQuest {
  pub fn new(quest: &Quest) -> AcceptedQuest {
    AcceptedQuest{ quest_id: quest.id(), name: quest.name().clone(), monsters_killed: 0 }
  }

  pub fn quest_id(&self) -> i32 {
    self.quest_id
  }
}

pub struct PlayerState {
  username: String,
  accepted_quests: Vec<AcceptedQuest>
}

impl PlayerState {
  pub fn new(username: String) -> PlayerState {
    PlayerState { username: username, accepted_quests: vec![] }
  } 

  pub fn username(&self) -> &String {
    return &self.username;
  }

  pub fn accept_quest(&mut self, quest: &Quest) {
    println!("accepting quest {}", quest.id());
    self.accepted_quests.push(AcceptedQuest::new(quest))
  }

  pub fn accepted_quests(&self) -> &Vec<AcceptedQuest> {
    &self.accepted_quests
  }
}

pub type TsPlayerState = Ts<PlayerState>;

impl<'a, 'r> FromRequest<'a, 'r> for TsPlayerState {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<TsPlayerState, ()> {
        let cookies = request.guard::<Cookies>()?;
        
        match cookies.get("id") {
          Some(cookie) => {
            let state = request.guard::<State<game_state::TsGameState>>()?;
            let lock = state.read();
            match lock.get_player(cookie.value()) {
                Some(player) => {
                  Outcome::Success(player)
                },
                None => Outcome::Forward(())
            }
          },
          None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}