/// 
/// Get all the todo items from the database
/// 
pub const SELECT_QUERY: &str = "SELECT id, title, completed FROM todo";

///
/// Get a todo item from the database by id
/// 
pub const SELECT_BY_ID_QUERY: &str = "SELECT id, title, completed FROM todo WHERE id = $1";

///
/// Insert a todo item into the database
/// 
pub const INSERT_QUERY: &str =
    "INSERT INTO todo (title, completed) VALUES ($1, $2) RETURNING id, title, completed";

///
/// Update a todo item in the database
/// 
pub const UPDATE_QUERY: &str =
    "UPDATE todo SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed";

///
/// Delete a todo item from the database
/// 
pub const DELETE_QUERY: &str = "DELETE FROM todo WHERE id = $1";
