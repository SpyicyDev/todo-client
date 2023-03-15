mod sql;
mod components;

use leptos::*;
use crate::components::*;

// use crate::sql::sql;

use futures::executor::*;

fn main() {
    mount_to_body(|cx| view! {cx,
        <TodoList/>
    });

    /*
    let res = sql(0, "", 2).unwrap().unwrap();
    res.iter().for_each(
        |todo| println!("id: {}, task: {}", todo.todo_id, todo.todo)
    );
     */
}