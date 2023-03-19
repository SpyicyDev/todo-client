use leptos::*;
use crate::server::{add_todo, delete_todo, get_all_todos, get_count, inc_count, TodoItem};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use wasm_bindgen_futures::JsFuture;

pub async fn timer(ms: i32) -> Result<(), JsValue> {
    let promise = Promise::new(&mut |yes, _| {
        let win = window();
        win.set_timeout_with_callback_and_timeout_and_arguments_0(&yes, ms)
            .unwrap();
    });
    let js_fut = JsFuture::from(promise);
    js_fut.await?;
    Ok(())
}

#[component]
pub fn TodoList(cx: Scope) -> impl IntoView {
    let todo_updater = create_rw_signal(cx, 0);

    let todos: Resource<i32, Vec<TodoItem>> = create_local_resource(cx, todo_updater, get_all_todos);

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            timer(2000).await.unwrap();
            todo_updater.update(|todo_updater| *todo_updater += 1);
        }
    });

    let todos_view = move || {
        todos.with(cx, |data| {
            data.iter()
                .map(|item| view! {cx,
                    <TodoItem id=item.todo_id text=item.todo_text.clone() todo_updater=todo_updater/>
                })
                .collect::<Vec<_>>()
        })
    };

    view! {cx,
        <InputTodo todo_updater=todo_updater/>
        <div class="todo-container">
            {todos_view}
        </div>

    }
}

#[component]
pub fn TodoItem(cx: Scope, id: i32, text: String, todo_updater: RwSignal<i32>) -> impl IntoView {
    let deleter = create_rw_signal(cx, id);
    let delete_func = create_action(cx, |&input| delete_todo(input));
    let delete = move |_| {
        delete_func.dispatch(deleter.get());
        todo_updater.update(|todo_updater| *todo_updater += 1);
    };

    let loading_view = move || {
        view! {cx,
            <div class="loader"></div>
        }
    };

    view! {cx,
        <div class="todo-item">
            <p>{text}</p>
            <div class="delete-container">
                {move || delete_func.pending().get().then(loading_view)}
                <button class="my-buttons" on:click=delete>"X"</button>
            </div>
        </div>
    }
}

#[component]
pub fn InputTodo(cx: Scope, todo_updater: RwSignal<i32>) -> impl IntoView {
    let adder = create_rw_signal(cx, (0, "".to_string()));

    let add_func = create_action(cx, |input: &(i32, String)| add_todo(input.clone()));
    let inc_func = create_action(cx, |&()| inc_count());

    let count_res = create_action(cx, |&()| { get_count() } );
    count_res.dispatch(());
    let mut count: i32 = 0;
    let mut update_count_on_load = 0;

    let elem: NodeRef<Input> = create_node_ref(cx);

    let add = move |ev: SubmitEvent| {
        ev.prevent_default();

        if update_count_on_load == 0 {
            count = count_res.value().get().unwrap();
            update_count_on_load = 1;
        }

        let value = elem()
            .expect("<input> to exist")
            .value();
        adder.update(|adder| adder.1 = value);

        // set current count
        adder.update(|adder| adder.0 = count);

        // add the task
        add_func.dispatch(adder.get());

        // empty input box
        elem().expect("<input> to exist").prop("value", "");

        // mutate count locally and on server
        inc_func.dispatch(());
        count += 1;

        todo_updater.update(|todo_updater| *todo_updater += 1);
    };

    let loading_view = move || {
        view! {cx,
            <div class="loader-add"></div>
        }
    };

    view! {cx,
        <div class="input-container">
            <form on:submit=add>
                <input id="text" type="text" class="input-box" name="todo-input" placeholder="Write a new to-do here" node_ref=elem/>
                <input id="btn" type="submit" style="display: none"/>
            </form>
            { move || add_func.pending().get().then(loading_view)}
        </div>
    }
}