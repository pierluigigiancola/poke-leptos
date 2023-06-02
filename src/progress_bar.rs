use leptos::*;

#[component]
pub fn ProgressBar<F>(
    cx: Scope,
    #[prop(optional)] label: &'static str,
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
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
