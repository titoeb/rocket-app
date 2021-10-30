use super::schema::users;
#[derive(Insertable, Queryable, serde::Serialize, serde::Deserialize, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
