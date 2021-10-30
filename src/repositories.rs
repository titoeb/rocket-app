use super::models::*;
use super::schema::*;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::result::QueryResult;
use rocket::response::status;
pub struct UsersRepository;

impl UsersRepository {
    pub fn load_all(c: &diesel::SqliteConnection) -> QueryResult<Vec<User>> {
        users::table.limit(100).load::<User>(c)
    }
    pub fn find(c: &diesel::SqliteConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }
    pub fn create(c: &diesel::SqliteConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)?;
        let last_id = Self::last_id(c);
        Self::find(c, last_id.unwrap())
    }
    pub fn last_id(c: &diesel::SqliteConnection) -> diesel::result::QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(c)
    }
    pub fn save(c: &diesel::SqliteConnection, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(user.id))
            .set((
                users::name.eq(user.name.to_owned()),
                users::email.eq(user.email.to_owned()),
            ))
            .execute(c)?;
        Self::find(c, user.id)
    }

    pub fn delete(c: &diesel::SqliteConnection, id: i32) -> QueryResult<usize> {
        match Self::find(c, id) {
            Ok(_) => diesel::delete(users::table.find(id)).execute(c),
            Err(e) => Err(e),
        }
    }
}
