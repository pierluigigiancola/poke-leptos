use leptos::*;

#[component]
fn ProgressBar(cx: Scope, progress: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        <progress
            max="50"
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
        <ProgressBar progress={count} />
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
