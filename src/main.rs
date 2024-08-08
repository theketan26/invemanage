#[macro_use] extern crate rocket;
use dotenv_codegen::dotenv;
use futures::StreamExt;
use rocket::get;

use tokio;
use mongodb::{bson::{doc, oid::ObjectId}, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Collection, Cursor};

mod repo;
mod models;


#[get("/greet")]
fn hello() -> String {
    String::from("Hello World!")
}


#[tokio::main]
async fn main() {
    let repo = repo
        ::Repo
        ::new(dotenv!("MONGO_DB"))
        .await;

    let test_data = repo.test_impl().await;

    let test_vec: Vec<Result<models::Test, mongodb::error::Error>> = test_data.collect().await;

    println!("{:?}", test_vec);

    let _ = rocket::build()
        .mount("/api", routes![hello])
        .launch()
        .await;
}
