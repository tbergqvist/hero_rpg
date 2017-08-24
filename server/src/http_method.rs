use serde;

pub enum HttpMethod {
  Get,
  Post,
}

impl serde::Serialize for HttpMethod {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
      serializer.serialize_str(
        match *self {
          HttpMethod::Get => "GET",
          HttpMethod::Post => "POST"
      })
    }
}