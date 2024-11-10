use super::demo_content::DemoContent;
use leptos::{component, view, IntoView};

stylance::import_style!(styles, "app.module.css");
// stylance::import_crate_style!(my_style, "src/components/app.module.css");
// stylance::import_style!(styles, "app.css");

#[component]
pub fn App() -> impl IntoView {
  view! {
    <section class=styles::app>
      <DemoContent />
    </section>
  }
}
