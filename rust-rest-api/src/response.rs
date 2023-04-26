use crate::model::{Todo, User, TodoList, TodoTodoList, TodoTodoListWithIncompleteCount};
use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[derive(Serialize, JsonSchema)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: Todo,
}

#[derive (Serialize, Debug, JsonSchema)]
pub struct SingleUserResponse {
    pub status: String,
    pub data: User,
}


#[derive (Serialize, Debug, JsonSchema)]
pub struct SingleTodoListResponse {
    pub status: String,
    pub data: TodoList,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct TodoListResponse {
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct TodoListListResponse {
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct TodoListListPriorityResponse {
    pub status: String,
    pub data: Vec<String>,
    pub priorities: Vec<i32>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct UserListResponse {
    pub status: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct SingleTodoTodoListResponse {
    pub status: String,
    pub data: TodoTodoList, 
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct TodoTodoListListResponse {
    pub status: String,
    pub data: Vec<TodoTodoList>,
}


#[derive(Serialize, Debug, Deserialize, JsonSchema)]
pub struct TodoTodoListListWithIncompleteCountResponse {
    pub status: String,
    pub data: Vec<TodoTodoListWithIncompleteCount>,
}


