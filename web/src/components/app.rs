use super::demo_content::DemoContent;
use leptos::{component, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
  view! {
    <section class="some class">
      <DemoContent />
    </section>
  }
}
