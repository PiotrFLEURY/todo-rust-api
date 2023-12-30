
use serde_json::{
    json,
    Value,
};
use axum::extract::Path;
use axum::Json;
use crate::repository::{find_all, find_by_id, insert_into, update_where, delete_from};
use crate::todo_model::Todo;


pub async fn get_todos() -> Json<Vec<Todo>> {
    println!("get_todos");
    Json(find_all())
}

pub async fn get_todo(
    Path(id): Path<i32>,
) -> Json<Value> {
    println!("get_todo");
    let todo = find_by_id(id);
    Json(json!(todo))
}
pub async fn create_todo(
    Json(payload): Json<Todo>,
) -> Json<Todo> {
    println!("create_todo");
    // update todolist
    insert_into(payload.title.clone(), payload.completed.clone());
    Json(payload)
}
pub async fn update_todo(
    Path(id): Path<i32>,
    Json(payload): Json<Todo>,
) -> Json<Todo> {
    println!("update_todo");
    let todo = update_where(id, payload.title.clone(), payload.completed.clone());
    Json(todo)
}
pub async fn delete_todo(
    Path(id): Path<i32>,
) {
    println!("delete_todo");
    delete_from(id);
}