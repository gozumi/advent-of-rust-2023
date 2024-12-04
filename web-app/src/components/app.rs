use super::answer::Answer;
use super::data_input::DataInput;
use super::header::Header;
use leptos::{component, create_signal, logging::log, view, IntoView};
use lib::days::day_01::execute_day_01;

stylance::import_style!(styles, "app.module.css");

#[component]
pub fn App() -> impl IntoView {
  let (data, setData) = create_signal(String::new());
  let on_data_change = Box::new(|value: Vec<String>| {
    log!("Value changed: {:#?}", value.clone());
    // setData(value);
  });

  // let foo = execute_day_01()

  view! {
    <section class=styles::app>
      <Header />
      <DataInput on_change=on_data_change />
      <Answer />
    </section>
    n
  }
}
