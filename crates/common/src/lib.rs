use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub const UNABLE_TO_PARSE_DATA: &'static str = "Unable to parse data";

#[derive(PartialEq, Debug, Properties, Serialize, Deserialize, Clone)]
pub struct ToDo {
    pub id: usize,
    pub todo_info: String,
    pub todo_date: String,
}

impl ToDo {
    pub fn new(todo_info: &str, todo_date: &str, id: usize) -> Self {
        Self {
            id,
            todo_info: todo_info.to_string(),
            todo_date: todo_date.to_string(),
        }
    }
}
