use super::answer::Answer;
use super::data_input::DataInput;
use super::header::Header;
use leptos::{component, logging::log, view, IntoView};

stylance::import_style!(styles, "app.module.css");

#[component]
pub fn App() -> impl IntoView {
  let on_data_change = Box::new(|value| log!("Value changed: {}", value));

  view! {
    <section class=styles::app>
      <Header />
      <DataInput on_change=on_data_change />
      <Answer />
    </section>
    n
  }
}
