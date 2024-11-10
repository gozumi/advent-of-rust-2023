mod components;

use components::app::App;
use leptos::{document, mount_to, view};
use wasm_bindgen::JsCast;

fn main() {
  console_error_panic_hook::set_once();

  let app_root = document().get_element_by_id("app-root").unwrap().unchecked_into();

  mount_to(app_root, || view! { <App /> });
}
