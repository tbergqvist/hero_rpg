#[derive(Clone)]
pub struct Quest {
  id: i32,
  name: String,
  kill_goal: i32,
  enemy_id: i32
}

impl Quest {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn kill_goal(&self) -> i32 {
    self.kill_goal
  }

  pub fn enemy_id(&self) -> i32 {
    self.enemy_id
  }
}

pub fn create_quests() -> Vec<Quest> {
  vec![
    Quest {
      id: 1,
      name: String::from("Kill Rats"),
      enemy_id: 1,
      kill_goal: 2
    }
  ]
}