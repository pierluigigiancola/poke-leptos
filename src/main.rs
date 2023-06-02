use leptos::*;

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(optional)] label: &'static str,
    #[prop(default = 100)] max: u16,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! { cx,
         <label>
            {label}
        </label>
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

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
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
