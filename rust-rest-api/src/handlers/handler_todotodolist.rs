use crate::{
    model::{Todo, TodoTodoList, TodoTodoListWithIncompleteCount, IdList},
    response::{GenericResponse, SingleTodoTodoListResponse,
                TodoTodoListListResponse, TodoTodoListListWithIncompleteCountResponse},
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


// # Get all todostodolists
//
// Returns a list of all todostodolists
#[openapi(tag="TodosTodoLists")]
#[get("/todostodolists")]
pub async fn todostodolists_list_handler() -> Result<Json<TodoTodoListListResponse>, Status> {
    use crate::schema::todostodolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todostodolists
        .load::<TodoTodoList>(connection)
        .expect("Error loading todostodolists");


    let res2 = res
        .clone()
        .into_iter()
        .collect::<Vec<TodoTodoList>>();

    let response = TodoTodoListListResponse {
        status: "success".to_string(), 
        data: res2,
    };

    Ok(Json(response))
}

// # Get all todostodolists with number of incomplete todos
//
// Returns a list of all todostodolists with number of incomplete todos, unordered
#[openapi(tag="TodosTodoLists")]
#[get("/todostodolists/unordered")]
pub async fn todostodolists_list_unordered_handler() -> Result<Json<TodoTodoListListWithIncompleteCountResponse>, Status> {
    // returns a list of todostodolists with number of incomplete todos
    use crate::schema::todostodolists::dsl::*;
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let res = todostodolists
        .load::<TodoTodoList>(connection)
        .expect("Error loading todostodolists");
    

    let mut obj_vec = vec![];

    fn id_exists(vec: &Vec<TodoTodoListWithIncompleteCount>, id: String) -> bool {
        for obj in vec {
            if obj.todolist_id == id {
                return true;
            }
        }
        false
    }

    for t in res{
        let complete = todos
            .find(t.tid.clone())
            .first::<Todo>(connection)
            .map(|todo| todo.completed)
            .expect("Error loading completed");

        match complete {
            true => {
                let cl = obj_vec.clone();
                if !id_exists(&cl, t.tlid.clone()) {

                    let obj = TodoTodoListWithIncompleteCount {
                        todolist_id: t.tlid.clone(),
                        incomplete_count: 0,
                    };
                    obj_vec.push(obj);
                }
            },
            false => {
                let cl = obj_vec.clone();
                if id_exists(&cl, t.tlid.clone()) {
                    for mut obj in obj_vec.iter_mut(){
                        if obj.todolist_id == t.tlid.clone() {
                            obj.incomplete_count += 1;
                        }
                    }
                } else {
                    let obj = TodoTodoListWithIncompleteCount {
                        todolist_id: t.tlid.clone(),
                        incomplete_count: 1,
                    };
                    obj_vec.push(obj);
                }
            }
        }
    } 

    Ok(Json(TodoTodoListListWithIncompleteCountResponse {
        status: "success".to_string(),
        data: obj_vec 
    }))
}


// # Get all todostodolists with number of incomplete todos
//
// Returns a list of all todostodolists with number of incomplete todos, unordered
#[openapi(tag="TodosTodoLists")]
#[get("/todostodolists/ordered")]
pub async fn todostodolists_list_ordered_handler() -> Result<Json<TodoTodoListListWithIncompleteCountResponse>, Status> {
    // returns a list of todostodolists ordered by the number of incomplete todos
    use crate::schema::todostodolists::dsl::*;
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let res = todostodolists
        .load::<TodoTodoList>(connection)
        .expect("Error loading todostodolists");
    

    let mut obj_vec = vec![];

    fn id_exists(vec: &Vec<TodoTodoListWithIncompleteCount>, id: String) -> bool {
        for obj in vec {
            if obj.todolist_id == id {
                return true;
            }
        }
        false
    }

    for t in res{
        let complete = todos
            .find(t.tid.clone())
            .first::<Todo>(connection)
            .map(|todo| todo.completed)
            .expect("Error loading completed");

        match complete {
            true => {
                let cl = obj_vec.clone();
                if !id_exists(&cl, t.tlid.clone()) {

                    let obj = TodoTodoListWithIncompleteCount {
                        todolist_id: t.tlid.clone(),
                        incomplete_count: 0,
                    };
                    obj_vec.push(obj);
                }
            },
            false => {
                let cl = obj_vec.clone();
                if id_exists(&cl, t.tlid.clone()) {
                    for mut obj in obj_vec.iter_mut(){
                        if obj.todolist_id == t.tlid.clone() {
                            obj.incomplete_count += 1;
                        }
                    }
                } else {
                    let obj = TodoTodoListWithIncompleteCount {
                        todolist_id: t.tlid.clone(),
                        incomplete_count: 1,
                    };
                    obj_vec.push(obj);
                }
            }
        }
    } 

    obj_vec.sort_by(|a, b| a.incomplete_count.cmp(&b.incomplete_count));

    Ok(Json(TodoTodoListListWithIncompleteCountResponse {
        status: "success".to_string(),
        data: obj_vec 
    }))
}


// # Get all todostodolists by todo id
//
// Returns a list of all todostodolists with the given todo id.
#[openapi(tag = "TodosTodoLists")]
#[get("/todostodolists/todos/<id>")]
pub async fn todostodolists_list_by_todo_handler(id: String) -> Result<Json<TodoTodoListListResponse>, Status> {
    use crate::schema::todostodolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todostodolists
        .filter(tid.eq(id))
        .load::<TodoTodoList>(connection)
        .expect("Error loading todostodolists");

    let response = TodoTodoListListResponse {
        status: "success".to_string(),
        data: res,
    };

    Ok(Json(response))
}

// # Get all todostodolists by todolist id
//
// Returns a list of all todostodolists with the given todolist id.
#[openapi(tag = "TodosTodoLists")]
#[get("/todostodolists/todolists/<id>")]
pub async fn todostodolists_list_by_todolist_handler(id: String) -> Result<Json<TodoTodoListListResponse>, Status> {
    use crate::schema::todostodolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todostodolists
        .filter(tlid.eq(id))
        .load::<TodoTodoList>(connection)
        .expect("Error loading todostodolists");

    let response = TodoTodoListListResponse {
        status: "success".to_string(),
        data: res,
    };

    Ok(Json(response))
}

// # Create a new todostodolist
//
// Creates a new todostodolist and returns the created todostodolist.
#[openapi(tag = "TodosTodoLists")]
#[post("/todostodolists", data = "<body>")]
pub async fn create_todotodolist_handler(
    body: Json<TodoTodoList>,
    ) -> Result<Json<SingleTodoTodoListResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todostodolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todostodolists
        .filter(tid.eq(body.tid.clone()))
        .filter(tlid.eq(body.tlid.clone()))
        .first::<TodoTodoList>(connection)
        .ok();

    match res{
        Some(_) => {
            return Err(Custom(Status::BadRequest, Json(GenericResponse {
                status: "error".to_string(),
                message: "TodoTodoList already exists".to_string(),
            })));
        }
        None => {}
    }

    let res2 = diesel::insert_into(todostodolists)
        .values(&body.into_inner())
        .get_result::<TodoTodoList>(connection)
        .expect("Error creating todostodolist");

    let response = SingleTodoTodoListResponse {
        status: "success".to_string(),
        data: res2,
    };

    Ok(Json(response))
}

// # Create multiple todostodolists
//
// Creates multiple todostodolists and returns the created todostodolists.
#[openapi(tag = "TodosTodoLists")]
#[post("/todostodolists/<id>", data = "<body>")]
pub async fn create_multiple_todotodolist_handler(
    id: String,
    body: Json<IdList>
    ) -> Result<Json<TodoTodoListListResponse>, Custom<Json<GenericResponse>>> {
    use crate::schema::todostodolists::dsl::*;  
    let connection = &mut establish_connection();
    let mut res = Vec::new();
    for i in body.into_inner().ids{
        let res2 = diesel::insert_into(todostodolists)
            .values(&TodoTodoList{
                tlid: id.clone(),
                tid: i,
            })
            .get_result::<TodoTodoList>(connection)
            .expect("Error creating todostodolist");
        res.push(res2);
    }
    let response = TodoTodoListListResponse {
        status: "success".to_string(),
        data: res,
    };
    Ok(Json(response))
}

// # Delete todostodolist
//
// Deletes a todostodolist and returns the deleted todostodolist.
#[openapi(tag = "TodosTodoLists")]
#[post("/todostodolists/delete", data = "<body>")]
pub async fn delete_todotodolist_handler(
    body: Json<TodoTodoList>,
    ) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todostodolists::dsl::*;

    let connection = &mut establish_connection();
    let res = todostodolists
        .filter(tid.eq(body.tid.clone()))
        .filter(tlid.eq(body.tlid.clone()))
        .first::<TodoTodoList>(connection)
        .ok();

    match res{
        Some(_) => {}
        None => {
            return Err(Custom(Status::BadRequest, Json(GenericResponse {
                status: "error".to_string(),
                message: "TodoTodoList does not exist".to_string(),
            })));
        }
    }

    match diesel::delete(todostodolists.filter(tid.eq(body.tid.clone())).filter(tlid.eq(body.tlid.clone())))
        .execute(connection) {
        Ok(_) => {
            return Ok(Json(GenericResponse {
                status: "success".to_string(),
                message: "TodoTodoList deleted".to_string(),
            }));
        }
        Err(_) => {
            return Err(Custom(Status::BadRequest, Json(GenericResponse {
                status: "error".to_string(),
                message: "TodoTodoList could not be deleted".to_string(),
            })));
        }
    }

}











