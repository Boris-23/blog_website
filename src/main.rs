/* 02/28/25 - Reading the documentation to 

get used to axum  */

//calling the axum crate
/*
 And programming the example from 

 03/02/25
 the documentation: https://docs.rs/axum/latest/axum/

 https://docs.rs/axum/latest/axum/extract/index.html
 One of the modules for sqlx: https://docs.rs/sqlx/latest/sqlx/sqlite/index.html

 Axum crate io: https://crates.io/crates/axum

 Added this from cargo tomol of the example: https://github.com/tokio-rs/axum/blob/main/examples/readme/Cargo.toml
*/ 
use axum::{
    //extract::Json,
    routing::{get,post},
    handler::Handler,
    http::StatusCode,
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]

//Axum crate io: https://crates.io/crates/axum
//Instead of using user, I'm creating entry 
struct Entry{


    name: String,
    title: String,
    date: String,
    comments: String,
}
//https://docs.rs/axum/latest/axum/extract/index.html
//instead of creating user, I'm using creating entry
#[derive(Deserialize)]
struct CreateEntry{
    name: String,
}

async fn CreateEntry(Json(payload): Json<CreateEntry>){}

//use serde::Deserialize;
#[tokio::main]
async fn main() {

    //tracing 
    tracing_subscriber::fmt::init();
    
    //build the application with a single route
    let app = Router::new()
    .route("/", get(root))
    .route("/users",post(CreateEntry));

    //run the app on hyper 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


//https://docs.rs/axum/latest/axum/extract/index.html

// learning how to do a basic handler 
async fn root() -> &'static str{

    "Welcome to the test webpage!";
     "Updates will be coming up shortly!"
}

//creating an async function for entry 
async fn create_entry(
    Json(payload): Json<CreateEntry>,
) -> (StatusCode, Json<Entry>){

        let entry = Entry{

         //fixed some strings 
        name: payload.name,
        title: String::from("Blog"),
        date: String::from("March 3rd"),
        comments: String::from("This is a test"),
        };

        (StatusCode::CREATED, Json(entry))
}