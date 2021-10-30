#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::diesel::RunQueryDsl;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use models::*;
use rocket::response::status;
use schema::*;
#[macro_use]
use rocket::serde::json::{json, Value, Json};

mod auth;
mod models;
mod schema;
use auth::BasicAuth;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(auth: BasicAuth, conn: DbConn) -> Value {
    conn.run(|c| {
        let all = users::table
            .limit(100)
            .load::<User>(c)
            .expect("Error loading Users from db");
        json!(all)
    })
    .await
}

#[get("/users/<id>")]
async fn view_user(id: i32, auth: BasicAuth, conn: DbConn) -> Value {
    conn.run(move |c| {
        let user = users::table
            .find(id)
            .get_result::<User>(c)
            .expect("Could not load user!");
        json!(user)
    })
    .await
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(auth: BasicAuth, conn: DbConn, new_user: Json<NewUser>) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(users::table)
            .values(new_user.into_inner())
            .execute(c)
            .expect("Error adding user to DB");
        json!(result)
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<user>")]
async fn update_user(id: i32, auth: BasicAuth, conn: DbConn, user: Json<User>) -> Value {
    conn.run(move |c| {
        let result = diesel::update(users::table.find(id))
            .set((
                users::name.eq(user.name.to_owned()),
                users::email.eq(user.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating user {:?} into DB.");
        json!(result)
    })
    .await
}

#[delete("/users/<id>")]
async fn delete_user(id: i32, auth: BasicAuth, conn: DbConn) -> status::NoContent {
    conn.run(move |c| {
        diesel::delete(users::table.find(id))
            .execute(c)
            .expect("Could not delete.");
        status::NoContent
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
