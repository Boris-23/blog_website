/* 02/28/25 - Reading the documentation to 

get used to axum  */

//calling the axum crate
/*
 And programming the example from 
 the documentation: https://docs.rs/axum/latest/axum/

*/ 
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    
    //build the application with a single route
    let app = Router::new().route("/", get(|| async {"Hello, World"} ));

    //run the app on hyper 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
