use leptos::*;

#[component]
pub fn TodoList(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! {cx,
        <p> {count} </p>
    }
}

#[component]
pub fn TodoItem(cx: Scope) -> impl IntoView {
    // fill
    view! {cx,

    }
}