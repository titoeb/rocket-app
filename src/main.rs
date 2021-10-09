#[macro_use]
extern crate rocket;
use rocket::response::status;
use rocket::serde::json::{json, Value};

mod auth;
use auth::BasicAuth;

#[get("/users")]
fn get_users(auth: BasicAuth) -> Value {
    json!([
        {"id": 1, "name": "John Doe"}, {"id": 2, "name": "betrix 2"}
    ])
}

#[get("/users/<id>")]
fn view_user(id: i32, auth: BasicAuth) -> Value {
    json!([
        {"id": 1, "name": "John Doe", "email": "John@Dow.com"}, {"id": 2, "name": "betrix 2", "email": "beatrix@2.com"}
    ])
}

#[post("/users", format = "json")]
fn create_user(auth: BasicAuth) -> Value {
    json!([
        {"id": 1, "name": "John Doe", "email": "John@Dow.com"}, {"id": 2, "name": "betrix 2", "email": "beatrix@2.com"}
    ])
}

#[put("/users/<id>", format = "json")]
fn update_user(id: i32, auth: BasicAuth) -> Value {
    json!([
        {"id": 1, "name": "John Doe", "email": "John@Dow.com"}, {"id": 2, "name": "betrix 2", "email": "beatrix@2.com"}
    ])
}

#[delete("/users/<id>")]
fn delete_user(id: i32, auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn not_authorized() -> Value {
    json!("Not Authorized!")
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![get_users, view_user, create_user, update_user, delete_user]).register(catchers![not_found])
// }
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_users, view_user, create_user, update_user, delete_user],
        )
        .register("/", catchers![not_found, not_authorized])
        .launch()
        .await;
}
