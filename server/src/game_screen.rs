use serde;
use serde::ser::SerializeStruct;
use http_method::HttpMethod;

#[derive(Serialize)]
pub struct GameScreen {
  pub message: String,
  pub actions: Vec<GameAction>
}

pub fn add_logout(mut game_screen: GameScreen, base_url: &String) -> GameScreen {
  game_screen.actions.push(
    GameAction {
      name: String::from("Logout"),
      method: HttpMethod::Post,
      fields: vec![],
      link: format!("{}/logout", base_url),
    }
  );

  game_screen
}

#[derive(Serialize)]
pub struct GameAction {
  pub name: String,
  pub method: HttpMethod,
  pub link: String,
  pub fields: Vec<Field>
}

pub enum FieldValue {
  Text(Option<String>),
  //Number(Option<i32>),
}

pub struct Field {
  pub name: String,
  pub value: FieldValue
}

impl serde::Serialize for Field {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
      let mut state = serializer.serialize_struct("field", 3).unwrap();
      state.serialize_field("name", &self.name).unwrap();

      match self.value {
        FieldValue::Text(ref value) => {
          state.serialize_field("type", "text").unwrap();
          match value {
            &Option::Some(ref val) => state.serialize_field("value", val).unwrap(),
            &Option::None => state.serialize_field("value", "").unwrap()
          }
        },
        /*
        FieldValue::Number(value) => {
          state.serialize_field("type", "number").unwrap();
          match value {
            Option::Some(val) => state.serialize_field("value", &val).unwrap(),
            Option::None => state.serialize_field("value", "").unwrap()
          }
        }
        */
      };

      state.end()
    }
}