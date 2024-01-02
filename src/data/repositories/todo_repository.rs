use crate::data::{
    models::todo_model::Todo,
    queries::sql_queries::{
        DELETE_QUERY, INSERT_QUERY, SELECT_BY_ID_QUERY, SELECT_QUERY, UPDATE_QUERY,
    },
};
use postgres::{Client, NoTls};

///
/// Get a database connection
///
pub fn client() -> Client {
    let connection_string = "postgresql://postgres:postgres@127.0.0.1:5432/postgres";
    Client::connect(connection_string, NoTls).unwrap()
}

///
/// Find all the todo items from the database
///
pub fn find_all() -> Vec<Todo> {
    std::thread::spawn(move || {
        let mut todos = Vec::new();
        for row in client().query(SELECT_QUERY, &[]).unwrap() {
            let todo = Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get(2),
            };
            todos.push(todo);
        }
        todos
    })
    .join()
    .unwrap()
}

///
/// Find a todo item from the database by id
///
pub fn find_by_id(id: i32) -> Todo {
    std::thread::spawn(move || {
        let row = client().query_one(SELECT_BY_ID_QUERY, &[&id]).unwrap();
        Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get(2),
        }
    })
    .join()
    .unwrap()
}

///
/// Insert a todo item into the database
///
pub fn insert_into(title: String, completed: bool) -> Todo {
    std::thread::spawn(move || {
        let row = client()
            .query_one(INSERT_QUERY, &[&title, &completed])
            .unwrap();
        Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get(2),
        }
    })
    .join()
    .unwrap()
}

///
/// Update a todo item in the database
///
pub fn update_where(id: i32, title: String, completed: bool) -> Todo {
    std::thread::spawn(move || {
        let row = client()
            .query_one(UPDATE_QUERY, &[&title, &completed, &id])
            .unwrap();
        Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get(2),
        }
    })
    .join()
    .unwrap()
}

///
/// Delete a todo item from the database
///
pub fn delete_from(id: i32) {
    std::thread::spawn(move || {
        client().execute(DELETE_QUERY, &[&id]).unwrap();
    })
    .join()
    .unwrap()
}
