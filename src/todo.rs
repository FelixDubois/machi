use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub done: bool,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todos {
    pub name: String,
    pub todo_list: Vec<Todo>,
}
