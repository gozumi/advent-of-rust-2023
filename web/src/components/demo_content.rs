use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn DemoContent() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let double_count = move || count() * 2;

  view! {
    <div class="demo-content">
      <button
        on:click=move |_| {
          set_count.update(|n| *n += 1);
        }
        class=("button-20", move || count() % 2 == 1)
      >
        "Click me: "
        {move || count()}
      </button>
      <progress max=50 value=double_count />
      <p>"Double Count: "{double_count}</p>
    </div>
  }
}
