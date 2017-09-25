use serde;
use serde::ser::SerializeStruct;
use http_method::HttpMethod;

pub enum GuiItem {
  Text{value: String},
  Form{name: String, method: HttpMethod, link: String, fields: Vec<FieldItem> },
}

pub enum FieldItem {
  TextInput{ name: String, value: Option<String> },
}

impl serde::Serialize for GuiItem {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {

      match *self {
        GuiItem::Text{ref value} => {
          let mut state = serializer.serialize_struct("text", 3).unwrap();
          state.serialize_field("type", "text").unwrap();
          state.serialize_field("value", value).unwrap();
          state.end()
        },
        GuiItem::Form{ref name, ref method, ref link, ref fields} => {
          let mut state = serializer.serialize_struct("form", 5).unwrap();
          state.serialize_field("type", "form").unwrap();
          state.serialize_field("name", name).unwrap();
          state.serialize_field("method", method).unwrap();
          state.serialize_field("link", link).unwrap();
          state.serialize_field("fields", fields).unwrap();
          state.end()
      }
    }
  }
}

impl serde::Serialize for FieldItem {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
      let mut state = serializer.serialize_struct("field", 3).unwrap();

      match *self {
        FieldItem::TextInput{ref name, ref value} => {
          state.serialize_field("type", "text").unwrap();
          state.serialize_field("name", name).unwrap();
          match value {
            &Option::Some(ref val) => state.serialize_field("value", val).unwrap(),
            &Option::None => state.serialize_field("value", "").unwrap()
          }
        },
      };

      state.end()
    }
}