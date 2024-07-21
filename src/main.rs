extern crate dotenv_codegen;
use dotenv_codegen::dotenv;

#[macro_use] extern crate rocket;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

use rocket::get;


#[get("/greet")]
fn hello() -> String {
    String::from("Hello World!")
}


#[tokio::main]
async fn main() {
    let mut client_options = ClientOptions::parse(dotenv!("MONGO_DB")).await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    
    let client = Client::with_options(client_options).unwrap();
    
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .unwrap();
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let _ = rocket::build()
        .mount("/api", routes![hello])
        .launch()
        .await;
}
