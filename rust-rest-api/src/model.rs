use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::schema::{todos, users, todolists, todostodolists};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, JsonSchema)]
#[table_name = "todos"]
pub struct Todo {
    pub tid: String,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InsertableTodo {
    pub title: String,
    pub content: String,
}

#[derive (Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InsertableTodolist{
    pub title: String,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct IdList {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum TodoListPriority{
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, JsonSchema)]
#[table_name = "todolists"]
pub struct TodoList {
    pub tlid: String,
    pub uid: String,
    pub title: String,
    pub priority: i32,
    pub created_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, JsonSchema)]
#[table_name = "todostodolists"]
pub struct TodoTodoList {
    pub tid: String,
    pub tlid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, JsonSchema)]
#[table_name = "users"]
pub struct User {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateUserSchema {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct TodoTodoListWithIncompleteCount {
    pub todolist_id: String,
    pub incomplete_count: i64,
}

pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            todos: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
