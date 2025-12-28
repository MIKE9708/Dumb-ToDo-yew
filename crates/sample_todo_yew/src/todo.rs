use common::ToDo;
use gloo_net::http::Request;
use log::info;
use serde_json;
use yew::prelude::*;

/// Web Assembly is a sandboxed Enviorment so is not possible to read from  local storage
/// so we use a Redis istance to store our todo stuff
pub const SIMPLE_SERVER: &str = "http://127.0.0.1:3000";
pub const UNABLE_TO_PARSE_FROM_JSON: &'static str = "Unable to parse from Json";
pub const FAILED_TO_RETRIEVE_TODO: &'static str = "Unalble to retireve data";
pub const FAILED_TO_STORE_TODO: &'static str = "Unalble to store data";
pub const FAILED_TO_DELETE_TODO: &'static str = "Unalble to delete data";

pub const SIMPLE_SERVER_GET_TODO: &str = "/get_todo";
pub const SIMPLE_SERVER_DELETE_TODO: &'static str = "/delete_todo";
pub const SIMPLE_SERVER_STORE_TODO: &'static str = "/store_todo";

#[derive(PartialEq, Clone)]
pub enum FormState {
    Hidden,
    Visible(ActionType),
}

impl FormState {
    pub fn new() -> Self {
        Self::Hidden
    }

    pub fn show_add_form(&mut self) {
        *self = Self::Visible(ActionType::Add);
    }
}

// This Handle Only the actions for the Add-Update-Delete
#[derive(Clone, PartialEq)]
pub enum ActionType {
    Delete,
    Add,
    Update,
}

#[derive(Clone)]
pub enum Task {
    Delete(ToDo),
    Add(ToDo),
    Update(ToDo),
    Loaded(Vec<ToDo>),
}

impl Task {
    pub fn handle_task(&self, todos: std::rc::Rc<Vec<ToDo>>) -> std::rc::Rc<Vec<ToDo>> {
        match self {
            Self::Delete(delete_todo) => {
                let todos: std::rc::Rc<Vec<ToDo>> = std::rc::Rc::new(
                    todos
                        .iter()
                        .cloned()
                        .filter(|todo| delete_todo.id != todo.id)
                        .collect(),
                );

                return todos;
            }
            Self::Add(todo) => {
                let mut new_todos = (*todos).clone();
                new_todos.push(todo.clone());
                std::rc::Rc::new(new_todos)
            }
            Self::Update(update_todo) => {
                let todos = std::rc::Rc::new(
                    todos
                        .iter()
                        .cloned()
                        .map(|todo| {
                            if todo.id == update_todo.id {
                                return update_todo.clone();
                            } else {
                                return todo.clone();
                            }
                        })
                        .collect(),
                );

                return todos;
            }
            Self::Loaded(todos) => return todos.clone().into(),
        }
    }
}
pub enum TaskError {
    AddError,
    DeleteError,
    UpdateError,
    LoadError,
    GenericError(String),
}

impl TaskError {
    pub fn return_task_error(&self) -> String {
        match self {
            Self::DeleteError => "Failed to perform Delete Operation".to_string(),
            Self::AddError => "Failed to perform Add Operation".to_string(),
            Self::UpdateError => "Failed to perform Update Operation".to_string(),
            Self::LoadError => "Failed to retireve ToDos".to_string(),
            Self::GenericError(err) => err.clone(),
        }
    }
}

pub enum Msg {
    OnGoing(Task),
    Done(Task),
    Error(TaskError),
}

#[derive(PartialEq, Clone)]
pub struct ToDoState {
    pub todos: std::rc::Rc<Vec<ToDo>>,
    pub loading: bool,
    pub error: Option<String>,
}

impl ToDoState {
    pub fn new() -> Self {
        Self {
            todos: std::rc::Rc::new(vec![]),
            loading: true,
            error: None,
        }
    }
}

impl yew::Reducible for ToDoState {
    type Action = Msg;

    fn reduce(self: std::rc::Rc<Self>, msg: Self::Action) -> std::rc::Rc<Self> {
        match msg {
            Msg::OnGoing(_) => Self {
                loading: true,
                error: None,
                ..(*self).clone()
            }
            .into(),
            Msg::Done(task) => {
                let todos = task.handle_task(self.todos.clone());
                Self {
                    todos,
                    loading: false,
                    error: None,
                }
                .into()
            }

            Msg::Error(task_error) => Self {
                loading: false,
                error: Some(task_error.return_task_error()),
                ..(*self).clone()
            }
            .into(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ToDoListState {
    Loading,
    Loaded(Vec<ToDo>),
    ErrorToDo(String),
}

#[derive(PartialEq, Properties, Clone)]
pub struct ToDoListProps {
    pub state: UseReducerHandle<ToDoState>,
    pub form_state: UseStateHandle<FormState>,
    pub on_click: Callback<ToDo>,
    #[prop_or_default]
    pub on_close: Callback<()>,
}

pub async fn get_todo() -> Result<Vec<ToDo>, &'static str> {
    let path = format!("{}{}", SIMPLE_SERVER, &SIMPLE_SERVER_GET_TODO);
    let response = Request::get(&path)
        .send()
        .await
        .map_err(|_| FAILED_TO_RETRIEVE_TODO)?;

    info!("Wrong data for parsing: {:?}", response);
    let todo_list_props: Vec<ToDo> = response.json().await.map_err(|data| {
        info!("Wrong data for parsing: {}", data);
        UNABLE_TO_PARSE_FROM_JSON
    })?;

    Ok(todo_list_props)
}

pub async fn delete_todo(todo: &ToDo) -> Result<(), &'static str> {
    let todo_json = serde_json::to_string(todo).map_err(|_| common::UNABLE_TO_PARSE_DATA)?;

    let path = format!("{}{}", SIMPLE_SERVER, &SIMPLE_SERVER_DELETE_TODO);
    let resp = Request::post(&path)
        .header("Content-Type", "application/json")
        .body(todo_json)
        .send()
        .await
        .map_err(|_| FAILED_TO_DELETE_TODO)?;

    if resp.ok() {
        Ok(())
    } else {
        Err(FAILED_TO_DELETE_TODO)
    }
}
pub async fn store_todo(todo: &ToDo) -> Result<(), &'static str> {
    let todo_json = serde_json::to_string(todo).map_err(|_| common::UNABLE_TO_PARSE_DATA)?;

    let path = format!("{}{}", SIMPLE_SERVER, &SIMPLE_SERVER_STORE_TODO);
    let resp = Request::post(&path)
        .header("Content-Type", "application/json")
        .body(todo_json)
        .send()
        .await
        .map_err(|_| FAILED_TO_STORE_TODO)?;

    if resp.ok() {
        Ok(())
    } else {
        Err(FAILED_TO_STORE_TODO)
    }
}

// This Is for PlaceHolding must be changed later
pub async fn update_todo(todo: &ToDo) -> Result<(), &'static str> {
    let todo_json = serde_json::to_string(todo).map_err(|_| common::UNABLE_TO_PARSE_DATA)?;

    let path = format!("{}{}", SIMPLE_SERVER, &SIMPLE_SERVER_DELETE_TODO);
    let resp = Request::post(&path)
        .header("Content-Type", "application/json")
        .body(todo_json)
        .send()
        .await
        .map_err(|_| FAILED_TO_STORE_TODO)?;

    if resp.ok() {
        Ok(())
    } else {
        Err(FAILED_TO_DELETE_TODO)
    }
}

pub async fn manage_action_request(
    action_type: ActionType,
    todo: ToDo,
) -> Result<(), &'static str> {
    match action_type {
        ActionType::Add => store_todo(&todo).await,
        ActionType::Delete => delete_todo(&todo).await,
        ActionType::Update => update_todo(&todo).await,
    }
}
