use leptos::*;
use wasm_bindgen::JsCast;

#[component]
pub fn DataInput(on_change: Box<dyn Fn(String)>) -> impl IntoView {
  let (value, setValue) = create_signal(String::new());

  let handle_change = move |ev: web_sys::Event| {
    if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
    {
      let new_value = input.value();
      setValue(new_value.clone());
      on_change(new_value);
    }
  };

  view! {
    <textarea
      class="data-input"
      value=value()
      on:input=handle_change
      placeholder="Enter the data here..."
    />
  }
}
