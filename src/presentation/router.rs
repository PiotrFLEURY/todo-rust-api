use crate::presentation::handlers::todo_handler::{
    create_todo, delete_todo, get_todo, get_todos, update_todo,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
}
