#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod handlers;
mod model;
mod response;
mod schema;
mod db;


use crate::handlers::handler_admin::*;
use crate::handlers::handler_todo::*;
use crate::handlers::handler_todolist::*;
use crate::handlers::handler_todotodolist::*;
use crate::handlers::handler_user::*;

use rocket::{get, http::Status, http::Header, serde::json::Json, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};


#[derive(Serialize, JsonSchema)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::new();
    rocket::build().attach(CORS).manage(app_data).mount(
        "/",
        openapi_get_routes![
            todos_list_handler,
            create_todo_handler,
            get_todo_handler,
            edit_todo_handler,
            delete_todo_handler,
            users_list_handler,
            create_user_handler,
            get_user_handler,
            edit_user_handler,
            delete_user_handler,
            todolists_list_handler,
            todolists_list_by_user_handler,
            todolists_list_important_by_user_handler,
            todolists_list_priorites_by_user_handler,
            todolists_list_important_priorites_by_user_handler,
            create_todolist_handler,
            edit_todolist_handler,
            delete_todolist_handler,
            todolist_get_handler,
            todostodolists_list_handler,
            todostodolists_list_by_todo_handler,
            todostodolists_list_by_todolist_handler,
            create_todotodolist_handler,
            delete_todotodolist_handler,
            todostodolists_list_ordered_handler,
            todostodolists_list_unordered_handler,
            create_multiple_todotodolist_handler,
            delete_all_data,
            initialize_data,
            get_user_id_by_name_handler,
        ],
    )
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
    .mount(
        "/rapidoc/",
        make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                ..Default::default()
            },
            hide_show: HideShowConfig {
                allow_spec_url_load: false,
                allow_spec_file_load: false,
                ..Default::default()
            },
            ..Default::default()
        }),
    )
}

#[cfg(test)]
mod test{
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use crate::response::TodoTodoListListWithIncompleteCountResponse;

    #[test]
    fn test_todostodolists_list_unordered_handler(){
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/api/todostodolists/unordered").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_todostodolists_list_ordered_handler(){
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/api/todostodolists/ordered").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // let body_string = response.into_string().expect("valid body");
        // check that the body contains incomplete counts in the correct order
        
        let body: TodoTodoListListWithIncompleteCountResponse = response.into_json::<TodoTodoListListWithIncompleteCountResponse>().expect("valid json");
        let data = body.data;
        let mut incomplete_counts = data.iter().map(|x| x.incomplete_count).collect::<Vec<i64>>();
        for i in 0..incomplete_counts.len()-1{
            assert!(incomplete_counts[i] <= incomplete_counts[i+1]);
        }

    }
}
