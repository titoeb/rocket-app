#[macro_use]
extern crate diesel_migrations;
embed_migrations!();
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use models::*;
use repositories::UsersRepository;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
mod auth;
mod models;
mod repositories;
mod schema;
use auth::BasicAuth;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(_auth: BasicAuth, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        UsersRepository::load_all(c)
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/users/<id>")]
async fn view_user(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        UsersRepository::find(c, id)
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(
    _auth: BasicAuth,
    conn: DbConn,
    new_user: Json<NewUser>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        UsersRepository::create(c, new_user.into_inner())
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/users/<_id>", format = "json", data = "<user>")]
async fn update_user(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    user: Json<User>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        UsersRepository::save(c, user.into_inner())
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/users/<id>")]
async fn delete_user(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<status::NoContent, status::Custom<Value>> {
    conn.run(move |c| {
        UsersRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn not_authorized() -> Value {
    json!("Not Authorized!")
}

#[catch(422)]
fn unprocessable() -> Value {
    json!("Your JSON could not be parsed!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_users, view_user, create_user, update_user, delete_user],
        )
        .register("/", catchers![not_found, not_authorized, unprocessable])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
