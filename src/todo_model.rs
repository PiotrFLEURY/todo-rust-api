
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
