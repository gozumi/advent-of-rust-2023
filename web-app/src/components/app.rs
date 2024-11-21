use super::answer::Answer;
use super::data_input::DataInput;
use super::header::Header;
use leptos::{component, view, IntoView};

stylance::import_style!(styles, "app.module.css");

#[component]
pub fn App() -> impl IntoView {
  view! {
    <section class=styles::app>
      <Header />
      <DataInput />
      <Answer />
    </section>
  }
}
