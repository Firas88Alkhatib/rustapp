use crate::db::get_connection;
use crate::models::user::{NewUser, User, UserDto};
use crate::schema::users::dsl::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub async fn create_user(user: UserDto) -> User {
    let mut db: PgConnection = get_connection();

    let new_user: NewUser = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
    };
    let inserted_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut db)
        .expect("failed to create user");
    return inserted_user;
}
pub async fn get_user(user_id: i32) -> User {
    let mut db: PgConnection = get_connection();
    let user: User = users.find(user_id).first(&mut db).expect("failed to get user by id");
    return user;
}
pub async fn get_all_users() -> Vec<User> {
    let mut db: PgConnection = get_connection();
    let results: Vec<User> = users.load::<User>(&mut db).expect("failed to load all users");
    return results;
}
pub async fn delete_user(user_id: i32) {
    let mut connection: PgConnection = get_connection();
    diesel::delete(users.find(user_id))
        .execute(&mut connection)
        .expect("cannot delete user");
}
pub async fn update_user(user_id: i32, user: User) -> User {
    let mut db: PgConnection = get_connection();

    let updated_user: User = diesel::update(users.find(user_id))
        .set(&user)
        .get_result::<User>(&mut db)
        .expect("Failed to update user");
    return updated_user;
}
