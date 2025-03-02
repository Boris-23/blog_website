/* 02/28/25 - Reading the documentation to 

get used to axum  */

//calling the axum crate
/*
 And programming the example from 
 the documentation: https://docs.rs/axum/latest/axum/

 https://docs.rs/axum/latest/axum/extract/index.html

*/ 
use axum::{
    extract::Json,
    routing::{get,post},
    handler::Handler,
    Router,
};
use serde::Deserialize;


//https://docs.rs/axum/latest/axum/extract/index.html
//instead of creating user, I'm using creating entry
#[derive(Deserialize)]
struct CreateEntry{

    name: String,
    title: String,
    Date: String,
    Comments: String,
}

async fn CreateEntry(Json(payload): Json<CreateEntry>){}

//use serde::Deserialize;
#[tokio::main]
async fn main() {

    
    //build the application with a single route
    let app = Router::new().route("/", get(|| async {"Hello, World"} ))
    .route("/users",post(CreateEntry));

    //run the app on hyper 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
