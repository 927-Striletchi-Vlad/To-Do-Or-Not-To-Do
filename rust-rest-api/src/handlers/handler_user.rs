use crate::{
    model::{User, InsertableUser, UpdateUserSchema},
    response::{GenericResponse, SingleUserResponse, UserListResponse},
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

// # Get all users
//
// Returns a list of all users
#[openapi(tag = "Users")]
#[get("/users?<page>&<limit>")]
pub async fn users_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<UserListResponse>, Status>{
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let res = users
        .load::<User>(connection)
        .expect("Error loading users");


    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let res2 = res
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<User>>();

    let res3 = res2.into_iter().map(|user| user.uid).collect::<Vec<String>>();

    let json_response = UserListResponse {
        status: "ok".to_string(),
        data: res3,
    };

    Ok(Json(json_response))
}


// # Create a new user
//
// Creates a new user
#[openapi(tag = "Users")]
#[post("/users", data = "<body>")]
pub async fn create_user_handler(
    body: Json<InsertableUser>,
) -> Result<Json<SingleUserResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let res = users
        .load::<User>(connection)
        .expect("Error loading users");

    for user in res.iter(){
        if user.name == body.name{
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with name: '{}' already exists", user.name),
            };
            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();

    let new_user = User {
        uid: uuid_id.to_string(),
        name: body.name.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
        created_at : datetime.clone(),
    };

    let user = new_user.to_owned();

    let user_for_db = user.clone();
    let connection = &mut establish_connection();
    diesel::insert_into(users)
        .values(&user_for_db)
        .execute(connection)
        .expect("Error saving new user");

    let json_response = SingleUserResponse {
        status: "success".to_string(),
        data: user.clone(),
    };

    Ok(Json(json_response))
}


// # Get User
//
// Get a user by id
#[openapi(tag = "Users")]
#[get("/users/<id>")]
pub async fn get_user_handler(
    id: String,
) -> Result<Json<SingleUserResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let res = users
        .find(id.clone())
        .first::<User>(connection)
        .ok();

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("User with id: '{}' not found", id),
    };

    match res {
        Some(user) => {
            let json_response = SingleUserResponse {
                status: "success".to_string(),
                data: user,
            };
            Ok(Json(json_response))
        }
        None => Err(Custom(Status::NotFound, Json(error_response))),
    }
}


// # Get User by Name
//
// Get a user by name
#[openapi(tag = "Users")]
#[get("/users/name/<uname>")]
pub async fn get_user_id_by_name_handler(
    uname: String,
) -> Result<Json<String>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let res = users
        .filter(name.eq(uname.clone()))
        .first::<User>(connection)
        .ok();

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("User with name: '{}' not found", uname),
    };

    match res {
        Some(user) => {
            let response = user.uid;
            Ok(Json(response))
        }
        None => Err(Custom(Status::NotFound, Json(error_response))),
    }
} 


// # Update User
//
// Update a user
#[openapi(tag = "Users")]
#[post("/users/update/<id>", data = "<body>")]
pub async fn edit_user_handler(
    id: String,
    body: Json<UpdateUserSchema>,
) -> Result<Json<SingleUserResponse>, Custom<Json<GenericResponse>>>{

    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let res = users
        .find(id.clone())
        .first::<User>(connection)
        .ok();
    
    match res{
        None => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with id: '{}' not found", id),
            };
            Err(Custom(Status::NotFound, Json(error_response)))
        }

        Some(old_user) => {
            let time = Utc::now().naive_utc();
            let payload = User{
                uid: id.clone(),
                name: body.name.to_owned().unwrap_or(old_user.name.clone()),
                email: body.email.to_owned().unwrap_or(old_user.email.clone()),
                password: body.password.to_owned().unwrap_or(old_user.password.clone()),
                created_at: old_user.created_at.clone(),

            };

            let connection = &mut establish_connection();

            match diesel::update(users.find(id.clone()))
                .set(&payload)
                .execute(connection){
                    Ok(_) =>{
                        let json_response = SingleUserResponse {
                            status: "success".to_string(),
                            data: payload.clone(),
                        };
                        Ok(Json(json_response))
                    },
                    Err(_) => {
                        let error_response = GenericResponse {
                            status: "fail".to_string(),
                            message: format!("User with ID: {} not found", id),
                        };
                        Err(Custom(Status::NotFound, Json(error_response)))
                }
        }
    }

    }
}


// # Delete User
//
// Delete a user by ID
#[openapi(tag = "Users")]
#[post("/users/delete/<id>")]
pub async fn delete_user_handler(
    id: String,
) -> Result<Status, Custom<Json<GenericResponse>>> {

    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    match diesel::delete(users.find(id.clone())).execute(connection) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with ID: {} not found", id),
            };
            Err(Custom(Status::NotFound, Json(error_response)))
        }
    }
}

































































