use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use game_state;
use quest::Quest;
use rocket::http::Cookies;
use thread_safe::Ts;
use battle::Battle;
use enemy::Enemy;

pub struct AcceptedQuest {
  quest_id: i32,
  enemy_id: i32,
  name: String,
  enemies_killed: i32,
  req_enemies_killed: i32,
}

impl AcceptedQuest {
  pub fn new(quest: &Quest) -> AcceptedQuest {
    AcceptedQuest{ quest_id: quest.id(), enemy_id: quest.enemy_id(), name: quest.name().clone(), enemies_killed: 0, req_enemies_killed: quest.kill_goal() }
  }

  pub fn quest_id(&self) -> i32 {
    self.quest_id
  }
}

pub struct PlayerState {
  username: String,
  accepted_quests: Vec<AcceptedQuest>,
  pub current_battle: Option<Battle>,
}

pub enum BattleState<'a> {
  Fight(&'a Battle),
  End(BattleReward)
}

pub struct BattleReward {

}

impl PlayerState {
  pub fn new(username: String) -> PlayerState {
    PlayerState { username: username, accepted_quests: vec![], current_battle: None }
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

  pub fn init_battle(&mut self, enemy: Enemy) -> &Battle {
    self.current_battle = Some(Battle::new(enemy));
    self.current_battle.as_ref().unwrap()
  }

  fn on_enemy_killed(&mut self, enemy_id: i32) {
    for quest in self.accepted_quests.iter_mut().filter(|quest| {
      quest.enemy_id == enemy_id
    }) {
      quest.enemies_killed += 1;
      println!("quest progress! {}/{}", quest.enemies_killed, quest.req_enemies_killed);
    }
  }

  pub fn fight(&mut self) -> BattleState {
    let won = {
      let battle = self.current_battle.as_mut().unwrap();
      battle.do_damage(1);
      battle.enemy.current_hp <= 0
    };

    if won {
      let enemy_id = self.current_battle.as_mut().unwrap().enemy.id;
      self.on_enemy_killed(enemy_id);
      self.current_battle = None;
      BattleState::End(BattleReward{})
    } else {
      BattleState::Fight(self.current_battle.as_ref().unwrap())
    }
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
          None => Outcome::Forward(())
        }
    }
}