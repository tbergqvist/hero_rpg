use gui_item::GuiItem;
use http_method::HttpMethod;

pub fn get_logout_form(base_url: &String) -> GuiItem {
  GuiItem::Form {
    name: String::from("Logout"),
    method: HttpMethod::Post,
    fields: vec![],
    link: format!("{}/logout", base_url),
  }
}

pub fn back_to_village_form(base_url: &String) -> GuiItem {
  GuiItem::Form {
    name: format!("Back to village"),
    method: HttpMethod::Get,
    link: format!("{}/", base_url),
    fields: vec![]
  }
}