#[macro_use] extern crate rocket;

use rocket::get;


#[get("/greet")]
fn hello() -> String {
    String::from("Hello World!")
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![hello])
}
