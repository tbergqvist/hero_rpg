use enemy::Enemy;

#[derive(Clone)]
pub struct Battle {
  pub enemy: Enemy
}

pub enum BattleState {
  Fight(Battle),
  End(BattleReward),
  None
}

pub struct BattleReward {

}

impl Battle {
  pub fn new(enemy: Enemy) -> Battle {
    Battle{
      enemy: enemy
    }
  }

  pub fn do_damage(&mut self, damage: i32) {
    self.enemy.current_hp -= damage;
    /*
    if clone.enemy.current_hp <= 0 {
      BattleState::End(BattleReward{})
    } else {
      BattleState::Fight(clone)
    }
    */
  }
}