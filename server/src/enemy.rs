pub fn create_enemies() -> Vec<Enemy> {
  vec![
    Enemy {
      id: 1,
      name: String::from("Rat"),
      max_hp: 2,
      current_hp: 2,
    }
  ]
}

#[derive(Clone)]
pub struct Enemy {
  pub id: i32,
  pub name: String,
  pub max_hp: i32,
  pub current_hp: i32,
}

