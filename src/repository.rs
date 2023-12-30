use crate::todo_model::Todo;
use postgres::{Client, NoTls};

pub fn client() -> Client {
    let connection_string = "postgresql://postgres:postgres@127.0.0.1:5432/postgres";
    return Client::connect(connection_string, NoTls).unwrap();
}

pub fn find_all() -> Vec<Todo> {
    return std::thread::spawn(move || {
        let mut todos = Vec::new();
        for row in client()
            .query("SELECT id, title, completed FROM todo", &[])
            .unwrap()
        {
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
        let row = client()
            .query_one(
                "SELECT id, title, completed FROM todo WHERE id = $1",
                &[&id],
            )
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

pub fn insert_into(title: String, completed: bool) -> Todo {
    return std::thread::spawn(move || {
        let row = client()
        .query_one(
            "INSERT INTO todo (title, completed) VALUES ($1, $2) RETURNING id, title, completed",
            &[&title, &completed],
        )
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
        let row = client().query_one("UPDATE todo SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed", &[&title, &completed, &id]).unwrap();
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
        client()
            .execute("DELETE FROM todo WHERE id = $1", &[&id])
            .unwrap();                          
    })
    .join()
    .unwrap();
}
