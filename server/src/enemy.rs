pub fn create_enemies() -> Vec<Enemy> {
  vec![
    Enemy {
      id: 1,
      name: String::from("Rat"),
      hp: 100
    }
  ]
}

pub struct Enemy {
  id: i32,
  name: String,
  hp: i32
}
