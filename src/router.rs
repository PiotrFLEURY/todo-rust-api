use axum::{routing::{delete, get, post, put}, Router};
use crate::todo_handler::{get_todos, create_todo, get_todo, update_todo, delete_todo};


pub fn create_router() -> Router {

    return Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo));
}

