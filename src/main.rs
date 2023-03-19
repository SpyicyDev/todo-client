mod components;
mod server;

use leptos::*;
use crate::components::*;

fn main() {
    mount_to_body(|cx| view! {cx,
        <h1>"To-Do List"</h1>
        <TodoList/>
    });
}