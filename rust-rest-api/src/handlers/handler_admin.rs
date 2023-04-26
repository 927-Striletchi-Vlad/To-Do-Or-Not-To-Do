use crate::db::establish_connection;
use crate::model::*;
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json,
    serde::Deserialize
};
use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::json::from_str;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};


// # Delete all data from all tables
//
// This is for testing purposes only
#[openapi(tag = "Admin")]
#[delete("/data")]
pub fn delete_all_data() -> Custom<String> {

    use crate::schema::users::dsl::*;
    use crate::schema::todos::dsl::*;
    use crate::schema::todolists::dsl::*;
    use crate::schema::todostodolists::dsl::*;

    let conn = &mut establish_connection();
    let mut result = String::new();
    match diesel::delete(users).execute(conn){
        Ok(_) => result.push_str("Deleted all data from users table\n"),
        Err(_) => result.push_str("Failed to delete data from users table\n"),
    };
    match diesel::delete(todos).execute(conn){
        Ok(_) => result.push_str("Deleted all data from todos table\n"),
        Err(_) => result.push_str("Failed to delete data from todos table\n"),
    };
    match diesel::delete(todolists).execute(conn){
        Ok(_) => result.push_str("Deleted all data from todolists table\n"),
        Err(_) => result.push_str("Failed to delete data from todolists table\n"),
    };
    match diesel::delete(todostodolists).execute(conn){
        Ok(_) => result.push_str("Deleted all data from todostodolists table\n"),
        Err(_) => result.push_str("Failed to delete data from todostodolists table\n"),
    };
    Custom(Status::Ok, result)
}

pub fn initialize_data_users() ->String{
    use crate::schema::users::dsl::*;

    let json_users = r#"[
    {
    "name" : "Alice Smith",
    "email" : "alice.smith@gmail.com",
    "password" : "P@ssw0rd!"
    },

    {
    "name" : "Bob Johnson",
    "email" : "bob.johnson@yahoo.com",
    "password" : "MyP@ssw0rd!"
    },

    {
    "name" : "Sara Lee",
    "email" : "sara.lee@hotmail.com",
    "password" : "CakeIsYummy"
    },

    {
    "name" : "Alex Lee",
    "email" : "alex.lee@gmail.com",
    "password" : "IloveM@ths"
    },

    {
    "name" : "David Chen",
    "email" : "david.chen@gmail.com",
    "password" : "qwerty123"
    },

    {
    "name" : "Emily Davis",
    "email" : "emily.davis@yahoo.com",
    "password" : "MyDogIsCute!"
    },

    {
    "name" : "Grace Wang",
    "email" : "grace.wang@hotmail.com",
    "password" : "Summer2022"
    },

    {
    "name" : "Jack Brown",
    "email" : "jack.brown@gmail.com",
    "password" : "Football10"
    },

    {
    "name" : "Lily Jones",
    "email" : "lily.jones@yahoo.com",
    "password" : "Sunflowers123"
    },

    {
    "name" : "Michael Lee",
    "email" : "michael.lee@hotmail.com",
    "password" : "BeyonceFan1"
    }
    ]"#;

    #[derive(Deserialize)]
    struct UserNoId {
        name: String,
        email: String,
        password: String,
    }

    let conn = &mut establish_connection();
    let temp_users = from_str::<Vec<UserNoId>>(json_users).unwrap();
    let other_users: Vec<User> = temp_users.iter().map(|user| User {
        uid: Uuid::new_v4().to_string(),
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        created_at: Utc::now().naive_utc(),
        }).collect();
    let mut result = String::new();

    match diesel::insert_into(users)
        .values(&other_users)
        .execute(conn){
            Ok(_) => result.push_str("Users inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting users: {}", e)),
        };

    result
}

pub fn initialize_data_todos() -> String{

    use crate::schema::todos::dsl::*;
    let json_todos = r#"[

    {
    "title": "Buy groceries",
    "content": "Get milk, bread, eggs, cheese, and vegetables from the supermarket"
    },

    {
    "title": "Clean the bathroom",
    "content": "Scrub the toilet, sink, and shower. Sweep and mop the floor"
    },

    {
    "title": "Go for a run",
    "content": "Jog for 30 minutes around the park. Stretch before and after"
    },

    {
    "title": "Call mom",
    "content": "Ask her how she's doing and tell her about your week"
    },

    {
    "title": "Finish report",
    "content": "Write the conclusion section and proofread the entire document"
    },

    {
    "title": "Organize closet",
    "content": "Sort clothes by season, donate or sell items you no longer wear"
    },

    {
    "title": "Pay bills",
    "content": "Pay rent, electricity, water, and internet bills before the due date"
    },

    {
    "title": "Read book",
    "content": "Read two chapters of 'The Great Gatsby' before going to bed"
    },

    {
    "title": "Cook dinner",
    "content": "Make chicken stir-fry with rice and vegetables. Follow recipe instructions"
    },

    {
    "title": "Study for exam",
    "content": "Review notes, do practice questions, and memorize key concepts for biology exam"
    }
    ]"#;
    /*

    #[derive(Deserialize)]
    struct UserNoId {
        name: String,
        email: String,
        password: String,
    }

    let conn = &mut establish_connection();
    let temp_users = from_str::<Vec<UserNoId>>(json_users).unwrap();
    let other_users: Vec<User> = temp_users.iter().map(|user| User {
        uid: Uuid::new_v4().to_string(),
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        created_at: Utc::now().naive_utc(),
        }).collect();
    */
    
    #[derive(Deserialize)]
    struct TodoNoId {
        title: String,
        content: String,
    }
    let conn = &mut establish_connection();
    let temp_todos = from_str::<Vec<TodoNoId>>(json_todos).unwrap();
    let other_todos: Vec<Todo> = temp_todos.iter().map(|todo| Todo {
        tid: Uuid::new_v4().to_string(),
        title: todo.title.clone(),
        content: todo.content.clone(),
        created_at: Utc::now().naive_utc(),
        completed: false,
        updated_at: None,
        }).collect();
    let mut result = String::new();
    match diesel::insert_into(todos)
        .values(&other_todos)
        .execute(conn){
            Ok(_) => result.push_str("Todos inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting todos: {}", e)),
        };

    result
}

pub fn initialize_data_todolists() -> String {

    use crate::schema::todolists::dsl::*;
    let json_todolists = r#"[

    {
    "title": "Work",
    "priority": 9
    },

    {
    "title": "Household Chores",
    "priority": 7
    },

    {
    "title": "Fitness",
    "priority": 6
    },

    {
    "title": "Errands",
    "priority": 5
    },

    {
    "title": "Family",
    "priority": 8
    },

    {
    "title": "Self-Improvement",
    "priority": 7
    },

    {
    "title": "Social Life",
    "priority": 6
    },

    {
    "title": "Finances",
    "priority": 8
    },

    {
    "title": "Travel",
    "priority": 5
    },

    {
    "title": "Volunteering",
    "priority": 7
    }
    ]"#;
    
    fn get_user_ids() -> Vec<String> {
        use crate::schema::users::dsl::*;
        let conn = &mut establish_connection();
        let results = users.load::<User>(conn).expect("Error loading users");
        results.iter().map(|user| user.uid.clone()).collect()
    }

    let first_user_id = get_user_ids()[0].clone();

    #[derive(Deserialize)]
    struct TodoListNoId {
        title: String,
        priority: i32,
    }
    let temp_todolists = from_str::<Vec<TodoListNoId>>(json_todolists).unwrap();
    let other_todolists: Vec<TodoList> = temp_todolists.iter().map(|todolist| TodoList {
        uid: first_user_id.clone(),
        tlid: Uuid::new_v4().to_string(),
        title: todolist.title.clone(),
        priority: todolist.priority,
        created_at: Utc::now().naive_utc(),
        }).collect();
    let conn = &mut establish_connection();
    let mut result = String::new();
    match diesel::insert_into(todolists)
        .values(&other_todolists)
        .execute(conn){
            Ok(_) => result.push_str("Todolists inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting todolists: {}", e)),
        };

    result
}

pub fn initialize_data_todostodolists() -> String{

    fn get_todo_ids() -> Vec<String>{
        use crate::schema::todos::dsl::*;
        let conn = &mut establish_connection();
        let result = todos.select(tid).load::<String>(conn).unwrap();
        result
    }

    fn get_todolist_ids() -> Vec<String>{
        use crate::schema::todolists::dsl::*;
        let conn = &mut establish_connection();
        let result = todolists.select(tlid).load::<String>(conn).unwrap();
        result
    }

    use crate::schema::todostodolists::dsl::*;

    let conn = &mut establish_connection();
    let mut result = String::new();

    let todo_ids = get_todo_ids();
    let todolist_ids = get_todolist_ids();

    //put first 5 todos in 1st todolist, next 5 in 2nd todolist
    for i in 0..5{
        match diesel::insert_into(todostodolists)
            .values((tid.eq(todo_ids[i].clone()), tlid.eq(todolist_ids[0].clone())))
            .execute(conn){
                Ok(_) => result.push_str("Todolist-Todo inserted successfully"),
                Err(e) => result.push_str(&format!("Error inserting todolist-todo: {}", e)),
            };
    }

    for i in 5..10{
        match diesel::insert_into(todostodolists)
            .values((tid.eq(todo_ids[i].clone()), tlid.eq(todolist_ids[1].clone())))
            .execute(conn){
                Ok(_) => result.push_str("Todolist-Todo inserted successfully"),
                Err(e) => result.push_str(&format!("Error inserting todolist-todo: {}", e)),
            };
    }
    
    result
}

// # initialize database with some data
//
// This is for testing purposes only
#[openapi(tag="Admin")]
#[post("/data")]
pub fn initialize_data() -> Custom<String> {
    let mut result = String::new();
    result.push_str(&initialize_data_users());
    result.push_str(&initialize_data_todos());
    result.push_str(&initialize_data_todolists());
    result.push_str(&initialize_data_todostodolists());

    Custom(Status::Ok, result)
}



