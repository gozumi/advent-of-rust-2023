use leptos::*;
use wasm_bindgen::JsCast;

#[component]
pub fn DataInput(on_change: Box<dyn Fn(Vec<String>)>) -> impl IntoView {
  let (value, setValue) = create_signal(String::new());

  let handle_change = move |ev: web_sys::Event| {
    if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
    {
      let new_value = input.value();
      setValue(new_value.clone());
      on_change(new_value.lines().map(|l| l.trim().to_string()).collect::<Vec<String>>());
    }
  };

  view! {
    <textarea
      class="data-input"
      value=move || value()
      on:input=handle_change
      placeholder="Enter the data here..."
    />
  }
}
