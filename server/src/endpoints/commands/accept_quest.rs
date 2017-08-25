use game_screen;
use super::get_quests;

pub fn accept_quest() -> game_screen::GameScreen {
  get_quests::get_quests()
}