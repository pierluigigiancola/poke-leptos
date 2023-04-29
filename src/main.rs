use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button
            class:red=move || count.get() % 2 == 0
            class:blue=move || count.get() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
