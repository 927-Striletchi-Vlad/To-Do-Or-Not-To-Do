use crate::{
    model::{TodoList, InsertableTodolist},
    response::{GenericResponse, SingleTodoListResponse, TodoListListResponse, TodoListListPriorityResponse},
    db::establish_connection,
};

use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, 
};
use diesel::prelude::*;
use uuid::Uuid;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

// # Get all todolists
//
// Returns a list of all todolists
#[openapi(tag = "TodoLists")]
#[get("/todolists")]
pub async fn todolists_list_handler() -> Result<Json<TodoListListResponse>, Status> {
     
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoList>>();

    let res3 = res2.into_iter().map(|todolist| todolist.tlid).collect::<Vec<String>>();

    let json_response = TodoListListResponse {
        status: "ok".to_string(),
        data: res3,
    };

    Ok(Json(json_response))
}


// # Get todolist by user id
//
// Returns a list of all todolists for a user
#[openapi(tag = "TodoLists")]
#[get("/todolists/user/<userid>")]
pub async fn todolists_list_by_user_handler(userid: String) -> Result<Json<TodoListListResponse>, Status> {
     
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .filter(uid.eq(userid))
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoList>>();

    let res3 = res2.into_iter().map(|todolist| todolist.tlid).collect::<Vec<String>>();

    let json_response = TodoListListResponse {
        status: "ok".to_string(),
        data: res3,
    };

    Ok(Json(json_response))
}


// # Get important todolists by user id
//
// Returns a list of all important todolists (priority>5) for a user
#[openapi(tag = "TodoLists")]
#[get("/todolists/user/<userid>/important")]
pub async fn todolists_list_important_by_user_handler(userid: String) -> Result<Json<TodoListListResponse>, Status> {
     
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .filter(uid.eq(userid))
        .filter(priority.gt(5))
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoList>>();

    let res3 = res2.into_iter().map(|todolist| todolist.tlid).collect::<Vec<String>>();

    let json_response = TodoListListResponse {
        status: "ok".to_string(),
        data: res3,
    };

    Ok(Json(json_response))
}

// # Get todolist by user id, with priority
//
// Returns a list of all todolists for a user
#[openapi(tag = "TodoLists")]
#[get("/todolists/user/<userid>/withpriority")]
pub async fn todolists_list_priorites_by_user_handler(userid: String) -> Result<Json<TodoListListPriorityResponse>, Status> {
     
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .filter(uid.eq(userid))
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoList>>();

    struct TodoListPriority {
        tlid: String,
        priority: i32,
    }

    let res3 = res2.clone().into_iter().map(|todolist| todolist.tlid).collect::<Vec<String>>();
    let res4 = res2.into_iter().map(|todolist| todolist.priority).collect::<Vec<i32>>();

    let json_response = TodoListListPriorityResponse {
        status: "ok".to_string(),
        data: res3,
        priorities: res4,
    };

    Ok(Json(json_response))
}


// # Get important todolists by user id, with priority
//
// Returns a list of all important todolists for a user
#[openapi(tag = "TodoLists")]
#[get("/todolists/user/<userid>/importantwithpriority")]
pub async fn todolists_list_important_priorites_by_user_handler(userid: String) -> Result<Json<TodoListListPriorityResponse>, Status> {
     
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .filter(uid.eq(userid))
        .filter(priority.gt(5))
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoList>>();

    struct TodoListPriority {
        tlid: String,
        priority: i32,
    }

    let res3 = res2.clone().into_iter().map(|todolist| todolist.tlid).collect::<Vec<String>>();
    let res4 = res2.into_iter().map(|todolist| todolist.priority).collect::<Vec<i32>>();

    let json_response = TodoListListPriorityResponse {
        status: "ok".to_string(),
        data: res3,
        priorities: res4,
    };

    Ok(Json(json_response))
}


// # Get todolist by id
//
// Returns a single todolist
#[openapi(tag = "TodoLists")]
#[get("/todolists/<todolistid>")]
pub async fn todolist_get_handler(todolistid: String) -> Result<Json<SingleTodoListResponse>, Status> {
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .find(todolistid.clone())
        .first::<TodoList>(connection)
        .ok();

    let error_response = GenericResponse {
        status: "error".to_string(),
        message: "TodoList not found".to_string(),
    };

    match res{
        Some(todolist) => {
            let json_response = SingleTodoListResponse {
                status: "ok".to_string(),
                data: todolist,
            };
            Ok(Json(json_response))
        }
        None => Err(Status::NotFound)
    } 

}



// # Create todolist
//
// Creates a new todolist
#[openapi(tag = "TodoLists")]
#[post("/todolists/<userid>", data = "<body>")]
pub async fn create_todolist_handler(
    userid: String,
    body: Json<InsertableTodolist>,
    ) -> Result<Json<SingleTodoListResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .load::<TodoList>(connection)
        .expect("Error loading todolists");

    for t in res.iter(){
        if t.title == body.title {
            let json_response = GenericResponse {
                status: "error".to_string(),
                message: "Todolist with this title already exists".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(json_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();

    let new_todolist = TodoList {
        tlid: uuid_id.to_string(),
        uid: userid.clone(),
        title: body.title.clone(),
        priority: body.priority.clone(),
        created_at: datetime,
    };
    let todolist_for_db = new_todolist.clone();


    diesel::insert_into(todolists)
        .values(&todolist_for_db)
        .execute(connection)
        .expect("Error saving new todolist");

    let json_response = SingleTodoListResponse {
        status: "ok".to_string(),
        data: new_todolist.clone(),
    };

    Ok(Json(json_response))
}


// # Update todolist
//
// Update todolist with given id for given user
#[openapi(tag = "TodoLists")]
#[post("/todolists/update/<userid>/<todolistid>", data = "<body>")]
pub async fn edit_todolist_handler(
    userid: String,
    todolistid: String,
    body: Json<InsertableTodolist>,
    ) -> Result<Json<SingleTodoListResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todolists
        .find(todolistid.clone())
        .first::<TodoList>(connection)
        .ok();

    match res{
        None => {
            let json_response = GenericResponse {
                status: "error".to_string(),
                message: "Todolist with this id does not exist".to_string(),
            };
            Err(Custom(Status::NotFound, Json(json_response)))
        }

        Some(old_todo) => {
            let time = Utc::now().naive_utc();
            let payload = TodoList{
                tlid: old_todo.tlid,
                uid: old_todo.uid,
                title: body.title.clone(),
                priority: body.priority.clone(),
                created_at: time,
            };

            let connection = &mut establish_connection();
            match diesel::update(todolists.find(todolistid.clone()))
                .set(&payload)
                .execute(connection){
                    Ok(_) => {
                        let json_response = SingleTodoListResponse {
                            status: "ok".to_string(),
                            data: payload,
                        };
                        Ok(Json(json_response))
                    }
                    Err(_) => {
                        let json_response = GenericResponse {
                            status: "error".to_string(),
                            message: format!("Todolist with ID: {} not found", todolistid),
                        };
                        Err(Custom(Status::NotFound, Json(json_response)))
                }

            }
        }
    }
}


// # Delete a todolist
//
// Deletes a todolist from the database
#[openapi(tag = "TodoLists")]
#[post("/todolists/delete/<todolistid>")]
pub async fn delete_todolist_handler(
    todolistid: String,
    ) -> Result<Status, Custom<Json<GenericResponse>>> {
    
    use crate::schema::todolists::dsl::*;

    let connection = &mut establish_connection();

    match diesel::delete(todolists.find(todolistid.clone()))
        .execute(connection){
            Ok(_) => {
                Ok(Status::Ok)
            }
            Err(_) => {
                let json_response = GenericResponse {
                    status: "error".to_string(),
                    message: format!("Todolist with ID: {} not found", todolistid),
                };
                Err(Custom(Status::NotFound, Json(json_response)))
            }
        }
}




















