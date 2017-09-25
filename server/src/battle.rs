use enemy::Enemy;

#[derive(Clone)]
pub struct Battle {
  pub enemy: Enemy
}

impl Battle {
  pub fn new(enemy: Enemy) -> Battle {
    Battle{
      enemy: enemy
    }
  }

  pub fn do_damage(&mut self, damage: i32) {
    self.enemy.current_hp -= damage;
  }
}