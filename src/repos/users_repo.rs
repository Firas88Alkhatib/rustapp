use crate::db::get_connection;
use crate::models::user::{NewUser, User, UserDto};
use crate::schema::users::dsl::*;
use diesel::{result::Error, PgConnection, QueryDsl, RunQueryDsl};

pub async fn create_user(user: UserDto) -> Result<User, Error> {
    let mut db: PgConnection = get_connection();

    let new_user: NewUser = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
    };
    let inserted_user = diesel::insert_into(users).values(&new_user).get_result::<User>(&mut db);

    return inserted_user;
}
pub async fn get_user(user_id: i32) -> Result<User, Error> {
    let mut db: PgConnection = get_connection();
    let user = users.find(user_id).first(&mut db);
    return user;
}
pub async fn get_all_users() -> Result<Vec<User>, Error> {
    let mut db: PgConnection = get_connection();
    let results = users.load::<User>(&mut db);
    return results;
}
pub async fn delete_user(user_id: i32) -> Result<usize, Error> {
    let mut connection: PgConnection = get_connection();
    let result = diesel::delete(users.find(user_id)).execute(&mut connection);
    return result;
}
pub async fn update_user(user_id: i32, user: User) -> Result<User, Error> {
    let mut db: PgConnection = get_connection();

    let updated_user = diesel::update(users.find(user_id))
        .set(&user)
        .get_result::<User>(&mut db);

    return updated_user;
}
