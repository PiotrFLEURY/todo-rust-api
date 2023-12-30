use crate::todo_model::Todo;
use postgres::{Client, NoTls};

const SELECT_QUERY: &str = "SELECT id, title, completed FROM todo";
const SELECT_BY_ID_QUERY: &str = "SELECT id, title, completed FROM todo WHERE id = $1";
const INSERT_QUERY: &str =
    "INSERT INTO todo (title, completed) VALUES ($1, $2) RETURNING id, title, completed";
const UPDATE_QUERY: &str =
    "UPDATE todo SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed";
const DELETE_QUERY: &str = "DELETE FROM todo WHERE id = $1";

pub fn client() -> Client {
    let connection_string = "postgresql://postgres:postgres@127.0.0.1:5432/postgres";
    return Client::connect(connection_string, NoTls).unwrap();
}

pub fn find_all() -> Vec<Todo> {
    return std::thread::spawn(move || {
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
    .unwrap();
}

pub fn find_by_id(id: i32) -> Todo {
    return std::thread::spawn(move || {
        let row = client().query_one(SELECT_BY_ID_QUERY, &[&id]).unwrap();
        Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get(2),
        }
    })
    .join()
    .unwrap();
}

pub fn insert_into(title: String, completed: bool) -> Todo {
    return std::thread::spawn(move || {
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
    .unwrap();
}

pub fn update_where(id: i32, title: String, completed: bool) -> Todo {
    return std::thread::spawn(move || {
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
    .unwrap();
}

pub fn delete_from(id: i32) {
    return std::thread::spawn(move || {
        client().execute(DELETE_QUERY, &[&id]).unwrap();
    })
    .join()
    .unwrap();
}
