use leptos::{component, view, IntoView};

#[component]
pub fn DataInput() -> impl IntoView {
  view! {
    <div class="data-input">
      <textarea class="data-input__text" />
    </div>
  }
}
