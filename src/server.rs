use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub todo_id: i32,
    pub todo_text: String
}

pub async fn get_all_todos(_a: i32) -> Vec<TodoItem> {
    let res = reqwasm::http::Request::get("https://todo-api.mackhaymond.co/get-todos")
        .send()
        .await
        .unwrap()
        .json::<Vec<_>>()
        .await
        .unwrap();
    res
}

pub async fn add_todo(id_and_text: (i32, String)) {
    let id = id_and_text.0;
    let text = id_and_text.1.clone();
    reqwasm::http::Request::get(&format!("https://todo-api.mackhaymond.co/add-todo/{id}/{text}"))
        .send()
        .await
        .unwrap();
}

pub async fn delete_todo(id: i32) {
    reqwasm::http::Request::get(&format!("https://todo-api.mackhaymond.co/delete-todo/{id}",))
        .send()
        .await
        .unwrap();
}

pub async fn get_count() -> i32 {
    let res = reqwasm::http::Request::get("https://todo-api.mackhaymond.co/get-count")
        .send()
        .await
        .unwrap()
        .json::<_>()
        .await
        .unwrap();
    res
}

pub async fn inc_count() {
    reqwasm::http::Request::get(&format!("https://todo-api.mackhaymond.co/inc-count",))
        .send()
        .await
        .unwrap();
}