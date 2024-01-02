use crate::data::models::todo_model::Todo;
use crate::data::repositories::todo_repository::{
    delete_from, find_all, find_by_id, insert_into, update_where,
};
use axum::extract::Path;
use axum::Json;
use serde_json::{json, Value};

///
/// Get all the todo items and return them as JSON
///
pub async fn get_todos() -> Json<Vec<Todo>> {
    Json(find_all())
}

///
/// Get a todo item by id and return it as JSON
///
pub async fn get_todo(Path(id): Path<i32>) -> Json<Value> {
    let todo = find_by_id(id);
    Json(json!(todo))
}

///
/// Create a todo item and return it as JSON
///
pub async fn create_todo(Json(payload): Json<Todo>) -> Json<Todo> {
    insert_into(payload.title.clone(), payload.completed);
    Json(payload)
}

///
/// Update a todo item and return it as JSON
///
pub async fn update_todo(Path(id): Path<i32>, Json(payload): Json<Todo>) -> Json<Todo> {
    let todo = update_where(id, payload.title.clone(), payload.completed);
    Json(todo)
}

///
/// Delete a todo item
///
pub async fn delete_todo(Path(id): Path<i32>) {
    delete_from(id);
}
