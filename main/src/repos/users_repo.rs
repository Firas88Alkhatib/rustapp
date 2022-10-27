use crate::db::get_connection;
use crate::error_handle::{RepositoryError, RepositoryErrorType};
use crate::models::user::{NewUser, User, UserDto};
use crate::schema::users::dsl::*;
use crate::services::authentication_service::hash_password;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub async fn create_user(user: UserDto) -> Result<User, RepositoryError> {
    let mut db: PgConnection = get_connection();

    let hashed_password = hash_password(user.password).expect("unable to hash password");

    let new_user: NewUser = NewUser {
        username: &user.username,
        password: &hashed_password,
        roles: &user.roles,
        first_name: &user.first_name,
        last_name: &user.last_name,
    };

    let inserted_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut db)?;

    return Ok(inserted_user);
}
pub async fn get_all_users() -> Result<Vec<User>, RepositoryError> {
    let mut db: PgConnection = get_connection();
    let results = users.load::<User>(&mut db)?;
    return Ok(results);
}
pub async fn get_user(user_id: i32) -> Result<User, RepositoryError> {
    let mut db: PgConnection = get_connection();
    let user = users.find(user_id).first(&mut db)?;
    return Ok(user);
}

pub async fn get_user_by_username(user_name: String) -> Result<User, RepositoryError> {
    let mut db: PgConnection = get_connection();
    let user = users.filter(username.eq(user_name)).first(&mut db)?;
    return Ok(user);
}

pub async fn update_user(user_id: i32, user: User) -> Result<User, RepositoryError> {
    let mut db: PgConnection = get_connection();

    let updated_user = diesel::update(users.find(user_id))
        .set(&user)
        .get_result::<User>(&mut db)?;

    return Ok(updated_user);
}

pub async fn delete_user(user_id: i32) -> Result<usize, RepositoryError> {
    let mut connection: PgConnection = get_connection();
    let result = diesel::delete(users.find(user_id)).execute(&mut connection)?;
    return match result {
        0 => Err(RepositoryError {
            error_type: RepositoryErrorType::NotFound,
        }),
        _ => Ok(result),
    };
}
