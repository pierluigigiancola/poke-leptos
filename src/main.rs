mod progress_bar;
use leptos::*;
use progress_bar::{ProgressBar, ProgressBarProps};

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = move || count.get() * 2;

    view! { cx,
        <button
            style= move || format!("background-color: {}", if count() % 2 == 0  {"red"} else {"blue"})
            class:red=move || count() % 2 == 0
            class:blue=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>

        <div>
        <ProgressBar progress={count} />

        </div>
        <div>
        <ProgressBar progress={count} label="first progress bar" />

        </div>
        <div>
        <ProgressBar progress={count} max={50} label="Max 50" />
        </div>

        <div>
        <ProgressBar progress={double_count} max={50} label="Max 50 double count" />
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
