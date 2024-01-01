/// 
/// Todo model
/// 
/// Represents an item to do in the todo list
/// 
/// [id] - The id of the todo item
/// [title] - The title of the todo item
/// [completed] - Whether the todo item is completed or not
/// 
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct Todo {

    /// The id of the todo item
    pub id: i32,

    /// The title of the todo item
    pub title: String,

    /// Whether the todo item is completed or not
    pub completed: bool,
}
